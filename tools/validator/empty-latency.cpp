// MIT integration check using the official Steinberg hosting SDK (MIT).
#include "public.sdk/source/vst/hosting/module.h"
#include "public.sdk/source/vst/hosting/hostclasses.h"
#include "public.sdk/source/vst/hosting/plugprovider.h"
#include "pluginterfaces/vst/ivstaudioprocessor.h"
#include "pluginterfaces/vst/ivstcomponent.h"
#include <algorithm>
#include <iostream>
#include <stdexcept>
#include <vector>

using namespace Steinberg;
using namespace Steinberg::Vst;

static void require(bool ok, const char* message) {
    if (!ok) throw std::runtime_error(message);
}

int main(int argc, char** argv) {
    try {
        require(argc == 2, "Pass the VST3 bundle path");
        HostApplication host;
        PluginContextFactory::instance().setPluginContext(&host);
        std::string error;
        auto module = VST3::Hosting::Module::create(argv[1], error);
        if (!module) throw std::runtime_error("Module load failed: " + error);
        auto factory = module->getFactory();
        unsigned tested = 0;
        for (const auto& info : factory.classInfos()) {
            if (info.category() != kVstAudioEffectClass) continue;
            for (double rate : {44100.0, 48000.0, 96000.0}) {
                for (int32 block : {64, 512, 2048}) {
                    auto provider = owned(new PlugProvider(factory, info, true));
                    require(provider->initialize(), "Plugin initialization failed");
                    auto component = provider->getComponentPtr();
                    FUnknownPtr<IAudioProcessor> processor(component);
                    require(processor != nullptr, "Missing audio processor");
                    require(component->activateBus(kAudio, kOutput, 0, true) == kResultOk,
                            "Output activation failed");
                    ProcessSetup setup {kRealtime, kSample32, block, rate};
                    require(processor->setupProcessing(setup) == kResultOk, "Audio setup failed");
                    require(component->setActive(true) == kResultOk, "Activation failed");
                    const auto before = processor->getLatencySamples();
                    require(processor->setProcessing(true) == kResultOk, "Processing activation failed");
                    std::vector<float> left(block, 1.0f), right(block, 1.0f);
                    float* channels[] = {left.data(), right.data()};
                    AudioBusBuffers output {};
                    output.numChannels = 2; output.channelBuffers32 = channels;
                    ProcessData data {};
                    data.processMode = kRealtime; data.symbolicSampleSize = kSample32;
                    data.numSamples = block; data.numOutputs = 1; data.outputs = &output;
                    for (int i = 0; i < 4; ++i) {
                        require(processor->process(data) == kResultOk, "Empty audio process failed");
                        require(std::all_of(left.begin(), left.end(), [](float f) { return f == 0.0f; }) &&
                                std::all_of(right.begin(), right.end(), [](float f) { return f == 0.0f; }),
                                "Empty instance produced non-silent audio");
                    }
                    const auto after = processor->getLatencySamples();
                    processor->setProcessing(false);
                    component->setActive(false);
                    std::cout << "EMPTY_LATENCY rate=" << rate << " block=" << block
                              << " before=" << before << " after=" << after << " silent=true\n";
                    require(before == 0 && after == 0, "Empty instance still reports latency");
                    ++tested;
                }
            }
        }
        require(tested == 9, "Expected nine empty-instance audio configurations");
        std::cout << "Result: 9 empty VST3 latency checks passed\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
