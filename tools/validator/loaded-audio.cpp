// MIT integration diagnostic using the official Steinberg SDK (MIT).
// Loads real saved firmware into the actual VST3 and calls its audio processor.
// Define before SDK headers, which can transitively include Windows headers.
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include "public.sdk/source/vst/hosting/module.h"
#include "public.sdk/source/vst/hosting/hostclasses.h"
#include "public.sdk/source/vst/hosting/plugprovider.h"
#include "public.sdk/source/vst/hosting/eventlist.h"
#include "public.sdk/source/vst/hosting/parameterchanges.h"
#include "public.sdk/source/common/memorystream.h"
#include "pluginterfaces/vst/ivstaudioprocessor.h"
#include "pluginterfaces/vst/ivsteditcontroller.h"
#include "pluginterfaces/vst/ivstprocesscontext.h"
#include <algorithm>
#include <chrono>
#include <cmath>
#include <exception>
#include <stdexcept>
#include <fstream>
#include <iostream>
#include <iterator>
#include <thread>
#include <vector>
#ifdef _WIN32
#include <windows.h>
#endif
using namespace Steinberg;
using namespace Steinberg::Vst;
using Clock = std::chrono::steady_clock;
static void require(bool ok, const char* text) { if (!ok) throw std::runtime_error(text); }

int main(int argc, char** argv) {
    try {
        require(argc >= 5, "Usage: fm1-loaded-check bundle state.json block seconds [stopped|autoplay|daw|daw-hot]");
        const int block = std::stoi(argv[3]), seconds = std::stoi(argv[4]);
        require((block == 64 || block == 512) && seconds >= 12 && seconds <= 120,
                "Use block 64/512 and duration 12..120 seconds");
        const bool playing = argc < 6 || std::string(argv[5]) != "stopped";
        const bool daw = argc >= 6 && (std::string(argv[5]) == "daw" || std::string(argv[5]) == "daw-hot");
        const bool hot = argc >= 6 && std::string(argv[5]) == "daw-hot";
        const bool autoStarts = argc >= 6 && std::string(argv[5]) == "autoplay";
        std::ifstream input(argv[2], std::ios::binary);
        require(bool(input), "State fixture missing");
        std::vector<char> state((std::istreambuf_iterator<char>(input)), {});
        require(state.size() > 8 && state.size() < 4000000, "State fixture outside bound");
        uint64_t stateLength=0;
        for (unsigned i=0; i<8; ++i) stateLength|=uint64_t(static_cast<unsigned char>(state[i]))<<(8*i);
        require(stateLength==state.size()-8,"Invalid NIH-plug state length prefix");
        HostApplication host;
        PluginContextFactory::instance().setPluginContext(&host);
        std::string error;
        auto module = VST3::Hosting::Module::create(argv[1], error);
        if (!module) throw std::runtime_error(error);
        bool tested = false;
        for (const auto& info : module->getFactory().classInfos()) {
            if (info.category() != kVstAudioEffectClass) continue;
            auto provider = owned(new PlugProvider(module->getFactory(), info, true));
            require(provider->initialize(), "VST3 initialization failed");
            auto component = provider->getComponentPtr();
            FUnknownPtr<IAudioProcessor> processor(component);
            require(processor != nullptr, "No audio processor");
            auto controller = provider->getControllerPtr();
            require(controller != nullptr, "No controller");
            ParamID playId = kNoParamId;
            for (int32 i=0; i<controller->getParameterCount(); ++i) {
                ParameterInfo param{};
                require(controller->getParameterInfo(i, param) == kResultOk, "Parameter enumeration failed");
                if (param.title[0]=='P' && param.title[1]=='L' && param.title[2]=='A' && param.title[3]=='Y') playId=param.id;
            }
            require(playId != kNoParamId, "PLAY parameter not exposed");
            MemoryStream stream(state.data(), static_cast<TSize>(state.size()));
            require(component->setState(&stream) == kResultOk, "Firmware state restore failed");
            require(component->activateBus(kAudio, kOutput, 0, true) == kResultOk, "Audio bus activation failed");
            require(component->activateBus(kEvent, kInput, 0, true) == kResultOk, "MIDI bus activation failed");
            ProcessSetup setup{kRealtime, kSample32, block, 48000.0};
            require(processor->setupProcessing(setup) == kResultOk, "Audio setup failed");
            require(component->setActive(true) == kResultOk, "Activation failed");
            const auto latency = processor->getLatencySamples();
            std::exception_ptr failure;
            std::thread audio([&] {
                try {
#ifdef _WIN32
                    require(SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_TIME_CRITICAL) != 0, "Test audio priority failed");
                    auto timer = CreateWaitableTimerExW(nullptr, nullptr, 2, TIMER_MODIFY_STATE | SYNCHRONIZE);
                    require(timer != nullptr, "Test high-resolution timer unavailable");
                    struct Cleanup { HANDLE timer; ~Cleanup(){ CloseHandle(timer); } } cleanup{timer};
#endif
                    require(processor->setProcessing(true) == kResultOk, "Processing activation failed");
                    struct StopProcessing { IAudioProcessor* processor; ~StopProcessing(){ processor->setProcessing(false); } } stop{processor};
                    std::vector<float> left(block), right(block);
                    float* channels[]{left.data(), right.data()};
                    AudioBusBuffers output{}; output.numChannels=2; output.channelBuffers32=channels;
                    EventList events(64);
                    ParameterChanges changes;
                    ProcessContext context{};
                    context.sampleRate=48000.0; context.tempo=120.0;
                    context.timeSigNumerator=4; context.timeSigDenominator=4;
                    context.state=ProcessContext::kTempoValid | ProcessContext::kProjectTimeMusicValid
                        | ProcessContext::kTimeSigValid | (playing ? ProcessContext::kPlaying : 0);
                    ProcessData data{};
                    data.processMode=kRealtime; data.symbolicSampleSize=kSample32;
                    data.numSamples=block; data.numOutputs=1; data.outputs=&output;
                    data.inputEvents=&events; data.inputParameterChanges=&changes;
                    data.processContext=&context;
                    uint64_t active=0, silent=0, silentRun=0, longestSilent=0;
                    double peak=0.0, square=0.0;
                    double sections[12]{}; double sectionPeak[12]{};
                    std::chrono::nanoseconds callbackMax{}, lateMax{};
                    const auto started=Clock::now();
                    for (int64 frame=0; frame<int64(seconds)*48000; frame+=block) {
                        auto deadline=started+std::chrono::nanoseconds(frame*1000000000LL/48000);
#ifdef _WIN32
                        const auto remaining=deadline-Clock::now();
                        if (remaining>Clock::duration::zero()) {
                            LARGE_INTEGER due{};
                            due.QuadPart=-std::max<int64_t>(1, std::chrono::duration_cast<std::chrono::nanoseconds>(remaining).count()/100);
                            require(SetWaitableTimer(timer,&due,0,nullptr,nullptr,FALSE)!=0,"Host timer setup failed");
                            require(WaitForSingleObject(timer,1000)==WAIT_OBJECT_0,"Host timer wait failed");
                        }
#else
                        std::this_thread::sleep_until(deadline);
#endif
                        lateMax=std::max(lateMax,std::chrono::duration_cast<std::chrono::nanoseconds>(Clock::now()-deadline));
                        const bool dawPlaying = (hot || frame >= 96000) && (frame < 336000 || frame >= 432000);
                        if (daw) {
                            context.tempo = frame < 192000 ? 120.0 : 90.0;
                            context.state = (context.state & ~ProcessContext::kPlaying) | (dawPlaying ? ProcessContext::kPlaying : 0);
                            const auto elapsed = std::max<int64>(0, std::min<int64>(frame,336000) - (hot ? 0 : 96000));
                            const auto fast = std::min<int64>(elapsed,hot ? 192000 : 96000);
                            auto beat = fast/24000.0+(elapsed-fast)/32000.0;
                            if (frame >= 432000) beat += (frame-432000)/32000.0;
                            if (frame >= 504000) beat -= 4.0; // a host loop/seek
                            context.projectTimeSamples=frame;
                            context.projectTimeMusic=beat;
                        } else {
                            context.projectTimeSamples=playing ? frame : 0;
                            context.projectTimeMusic=playing ? frame/24000.0 : 0.0;
                        }
                        events.clear(); changes.clearQueue();
                        const auto contains=[&](int64 sample){ return frame<=sample && sample<frame+block; };
                        if (!daw && !autoStarts && (contains(96000) || contains(100800))) {
                            int32 index=0, point=0;
                            auto* q=changes.addParameterData(playId,index);
                            require(q && q->addPoint(0,contains(96000)?1.0:0.0,point)==kResultOk,"PLAY automation failed");
                        }
                        if (contains(144000) || contains(384000)) {
                            for (int channel=0; channel<16; ++channel) {
                                Event e{}; e.busIndex=0; e.sampleOffset=0;
                                if (contains(144000)) {
                                    e.type=Event::kNoteOnEvent; e.noteOn.channel=channel;
                                    e.noteOn.pitch=60; e.noteOn.velocity=0.8f; e.noteOn.noteId=-1;
                                } else {
                                    e.type=Event::kNoteOffEvent; e.noteOff.channel=channel;
                                    e.noteOff.pitch=60; e.noteOff.noteId=-1;
                                }
                                require(events.addEvent(e)==kResultOk,"MIDI enqueue failed");
                            }
                        }
                        const auto before=Clock::now();
                        output.silenceFlags=0;
                        require(processor->process(data)==kResultOk,"Loaded VST3 process failed");
                        callbackMax=std::max(callbackMax,std::chrono::duration_cast<std::chrono::nanoseconds>(Clock::now()-before));
                        for (auto* channel : channels) for (int i=0; i<block; ++i) {
                            const double v=channel[i]; require(std::isfinite(v),"Nonfinite audio");
                            if (frame/48000 < 12) { sections[frame/48000]+=v*v;
                                sectionPeak[frame/48000]=std::max(sectionPeak[frame/48000],std::abs(v)); }
                        }
                        if (frame>=192000) {
                            bool audible=false;
                            for (auto* channel : channels) for (int i=0; i<block; ++i) {
                                const double v=channel[i]; require(std::isfinite(v),"Nonfinite audio");
                                peak=std::max(peak,std::abs(v)); square+=v*v;
                                audible|=v!=0.0;
                            }
                            if (audible) { ++active; silentRun=0; }
                            else { ++silent; longestSilent=std::max(longestSilent,++silentRun); }
                        }
                    }
                    std::cout<<"LOADED_VST3 state="<<argv[2]<<" block="<<block<<" seconds="<<seconds
                        <<" playing="<<playing<<" daw="<<daw<<" hot="<<hot<<" latency="<<latency<<" active_blocks="<<active
                        <<" silent_blocks="<<silent<<" longest_silent_ms="<<longestSilent*block/48.0
                        <<" peak="<<peak<<" energy="<<square<<" callback_max_us="<<callbackMax.count()/1000
                        <<" host_late_max_us="<<lateMax.count()/1000<<std::endl;
                    require(peak>0.0001 && active>0,"Loaded firmware stayed silent");
                    if (daw) {
                        for (int i=0;i<12;++i) std::cout<<"DAW_SECTION second="<<i<<" energy="<<sections[i]<<" peak="<<sectionPeak[i]<<std::endl;
                        require(std::max(sectionPeak[3],sectionPeak[4])>0.0001,"No audio after DAW Start");
                        require(std::max(sectionPeak[10],sectionPeak[11])>0.0001,"No audio after DAW restart/loop");
                        require(sections[8] < sections[6]*0.1,"Arrangement kept playing after DAW Stop");
                        require(sections[10] > sections[8]*5.0,"Arrangement did not return after DAW restart/loop");
                    }
                } catch (...) { failure=std::current_exception(); }
            });
            audio.join();
            component->setActive(false);
            if (failure) std::rethrow_exception(failure);
            tested=true;
        }
        require(tested,"No instrument class tested");
        return 0;
    } catch (const std::exception& e) { std::cerr<<e.what()<<std::endl; return 1; }
}
