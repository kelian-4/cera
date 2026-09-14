
#include <cassert>
#include <iostream>

#include "model/Buffer.hpp"
#include "model/Row.hpp"

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

int main() {
    testRowInsertDelete();
    testRowTabExpansion();
    testRowCxRxRoundTrip();
    testBufferInsertNewlineSplitsRow();
    testBufferDeleteCharMergesRows();
    testBufferToString();
    testBufferDirtyFlag();

    std::cout << "Tous les tests du modèle sont passés.\n";
    return 0;
}
