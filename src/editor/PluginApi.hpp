#pragma once

#include <string>
#include <vector>

#include "model/Buffer.hpp"
#include "model/Cursor.hpp"

namespace cera::plugin {

struct PluginCommand {
    std::string id;
    std::string label;
    std::string shortcutHint;
};

class EditorApi {
public:
    virtual ~EditorApi() = default;

    virtual model::Buffer& activeBuffer() = 0;
    virtual const model::Buffer& activeBuffer() const = 0;
    virtual model::Cursor& cursor() = 0;

    virtual void setStatusMessage(const std::string& message) = 0;

    virtual void insertTextAtCursor(const std::string& text) = 0;

    virtual std::string clipboardPaste() = 0;
    virtual void clipboardCopy(const std::string& text) = 0;

    virtual int bufferCount() const = 0;
    virtual void switchToBuffer(int index) = 0;
};

class Plugin {
public:
    virtual ~Plugin() = default;

    virtual std::string name() const = 0;
    virtual std::string version() const = 0;

    virtual void onLoad(EditorApi& api) = 0;
    virtual void onUnload(EditorApi& api) = 0;

    virtual std::vector<PluginCommand> commands() const { return {}; }
    virtual void onCommand(EditorApi& api, const std::string& commandId) {
        (void)api;
        (void)commandId;
    }

    virtual void onBufferOpened(EditorApi& api, int bufferIndex) {
        (void)api;
        (void)bufferIndex;
    }
    virtual void onBeforeSave(EditorApi& api, int bufferIndex) {
        (void)api;
        (void)bufferIndex;
    }
    virtual void onKeyPress(EditorApi& api, int key, bool* handled) {
        (void)api;
        (void)key;
        *handled = false;
    }
};

}
