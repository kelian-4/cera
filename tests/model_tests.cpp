
#include <cassert>
#include <iostream>

#include "editor/BufferManager.hpp"
#include "editor/Clipboard.hpp"
#include "editor/UndoStack.hpp"
#include "io/Config.hpp"
#include "io/Json.hpp"
#include "model/Buffer.hpp"
#include "model/Row.hpp"
#include "model/Selection.hpp"
#include "model/Utf8.hpp"
#include "syntax/Syntax.hpp"

using cera::model::Buffer;
using cera::model::Highlight;
using cera::model::Row;

static void testRowInsertDelete() {
    Row row("helo");
    row.insertChar(3, 'l');
    assert(row.chars() == "hello");

    row.delChar(0);
    assert(row.chars() == "ello");
}

static void testRowTabExpansion() {
    Row row("a\tb");

    assert(row.render().size() == 9);
    assert(row.render()[0] == 'a');
    assert(row.render()[8] == 'b');
}

static void testRowCxRxRoundTrip() {
    Row row("a\tbc");
    int rx = row.cxToRx(3);
    int cx = row.rxToCx(rx);
    assert(cx == 3);
}

static void testRowUtf8() {
    Row row("a");
    row.insertChar(1, 0xE9);
    assert(row.size() == 2);
    assert(row.chars() == "a\xc3\xa9");

    row.delChar(1);
    assert(row.chars() == "a");
    assert(row.size() == 1);
}

static void testUtf8Utilities() {
    using namespace cera::model::utf8;
    std::string s = "a\xc3\xa9z";
    assert(codepointCount(s) == 3);
    assert(byteOffsetForCharIndex(s, 2) == 3);
    assert(charIndexForByteOffset(s, 3) == 2);
    assert(encode(0xE9) == "\xc3\xa9");
}

static void testBufferInsertNewlineSplitsRow() {
    Buffer buf;
    buf.insertRow(0, "helloworld");
    buf.insertNewline(0, 5);

    assert(buf.numRows() == 2);
    assert(buf.row(0).chars() == "hello");
    assert(buf.row(1).chars() == "world");
}

static void testBufferDeleteCharMergesRows() {
    Buffer buf;
    buf.insertRow(0, "hello");
    buf.insertRow(1, "world");

    auto pos = buf.deleteChar(1, 0);
    assert(buf.numRows() == 1);
    assert(buf.row(0).chars() == "helloworld");
    assert(pos.row == 0);
    assert(pos.col == 5);
}

static void testBufferToString() {
    Buffer buf;
    buf.insertRow(0, "line1");
    buf.insertRow(1, "line2");
    assert(buf.toString() == "line1\nline2\n");
}

static void testBufferDirtyFlag() {
    Buffer buf;
    assert(!buf.dirty());
    buf.insertRow(0, "x");
    assert(buf.dirty());
    buf.clearDirty();
    assert(!buf.dirty());
}

static void testUndoStackCoalescing() {
    using namespace cera::editor;
    UndoStack stack;

    EditAction a1;
    a1.kind = EditKind::InsertChar;
    a1.start = {0, 0};
    a1.insertedText = "a";
    a1.cursorBefore = {0, 0};
    a1.cursorAfter = {0, 1};
    stack.push(a1);

    EditAction a2;
    a2.kind = EditKind::InsertChar;
    a2.start = {0, 1};
    a2.insertedText = "b";
    a2.cursorBefore = {0, 1};
    a2.cursorAfter = {0, 2};
    stack.push(a2);

    assert(stack.canUndo());
    const EditAction* undone = stack.undo();
    assert(undone != nullptr);
    assert(undone->insertedText == "ab");

    assert(!stack.canUndo());
    assert(stack.canRedo());

    stack.breakCoalescing();
    EditAction a3;
    a3.kind = EditKind::InsertChar;
    a3.start = {0, 0};
    a3.insertedText = "c";
    a3.cursorBefore = {0, 0};
    a3.cursorAfter = {0, 1};
    stack.push(a3);
    assert(!stack.canRedo());
}

static void testJsonParsing() {
    using namespace cera::io::json;
    auto result = parse(R"({"a": 1, "b": "text", "c": [1,2,3], "d": true})");
    assert(result.ok);
    assert(result.value->get("a")->asInt() == 1);
    assert(result.value->get("b")->asString() == "text");
    assert(result.value->get("c")->arrayValue.size() == 3);
    assert(result.value->get("d")->asBool() == true);

    auto bad = parse("not json");
    assert(!bad.ok);
}

static void testClipboardFallback() {
    cera::editor::Clipboard clip;
    clip.copy("test clipboard content");
    assert(clip.paste() == "test clipboard content");
}

static void testSyntaxSelection() {
    auto* c = cera::syntax::selectForFilename("main.cpp");
    assert(c != nullptr && c->filetype == "c");
    auto* py = cera::syntax::selectForFilename("script.py");
    assert(py != nullptr && py->filetype == "python");
    auto* js = cera::syntax::selectForFilename("app.ts");
    assert(js != nullptr && js->filetype == "javascript");
    auto* none = cera::syntax::selectForFilename("data.xyz");
    assert(none == nullptr);
}

static void testBufferManagerMultipleBuffers() {
    cera::editor::BufferManager mgr;
    assert(mgr.count() == 1);
    mgr.openNewEmpty();
    assert(mgr.count() == 2);
    mgr.next();
    assert(mgr.activeIndex() == 0);
    mgr.previous();
    assert(mgr.activeIndex() == 1);
}

static void testSelectionRange() {
    cera::model::Selection sel;
    sel.anchor = {2, 5};
    sel.active = {1, 3};
    sel.isActive = true;

    auto start = sel.start();
    auto end = sel.end();
    assert(start.row == 1 && start.col == 3);
    assert(end.row == 2 && end.col == 5);
    assert(!sel.empty());
}

static void testConfigDefaults() {
    bool loaded = false;
    auto cfg = cera::io::Config::loadFromFile("/nonexistent/path.json", &loaded);
    assert(!loaded);
    assert(cfg.tabStop == 8);
    assert(cfg.quitTimes == 3);
}

int main() {
    testRowInsertDelete();
    testRowTabExpansion();
    testRowCxRxRoundTrip();
    testRowUtf8();
    testUtf8Utilities();
    testBufferInsertNewlineSplitsRow();
    testBufferDeleteCharMergesRows();
    testBufferToString();
    testBufferDirtyFlag();
    testUndoStackCoalescing();
    testJsonParsing();
    testClipboardFallback();
    testSyntaxSelection();
    testBufferManagerMultipleBuffers();
    testSelectionRange();
    testConfigDefaults();

    std::cout << "Tous les tests du modele sont passes.\n";
    return 0;
}
