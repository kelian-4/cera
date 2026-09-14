#include "view/ScreenRenderer.hpp"

#include <algorithm>
#include <cctype>
#include <cstdio>

#include "model/Utf8.hpp"
#include "view/Colors.hpp"
#include "view/HelpText.hpp"

namespace cera::view {

namespace {

int digitCount(int n) {
    if (n < 10) return 1;
    int count = 0;
    while (n > 0) {
        count++;
        n /= 10;
    }
    return count;
}

}

int ScreenRenderer::gutterWidth(const ViewState& state) const {
    if (!state.config.showLineNumbers) return 0;
    const auto& buf = state.bufferManager.active().buffer;
    int digits = digitCount(std::max(buf.numRows(), 1));
    return digits + 2;
}

int ScreenRenderer::textAreaScreenRows(const ViewState& state) const {
    int reserved = 2;
    if (state.bufferManager.count() > 1) reserved += 1;
    if (state.showHelpBar) reserved += 1;
    return std::max(state.screenrows - reserved, 1);
}

void ScreenRenderer::drawTabBar(AppendBuffer& ab, const ViewState& state) {
    if (state.bufferManager.count() <= 1) return;

    std::string line;
    for (int i = 0; i < state.bufferManager.count(); i++) {
        const auto& ob = state.bufferManager.at(i);
        std::string name = ob.buffer.filename() ? *ob.buffer.filename() : "[Sans nom]";
        size_t slash = name.find_last_of('/');
        if (slash != std::string::npos) name = name.substr(slash + 1);

        std::string label = " " + name;
        if (ob.buffer.dirty()) label += "*";
        label += " ";

        if (i == state.bufferManager.activeIndex()) {
            line += "[" + label + "]";
        } else {
            line += " " + label + " ";
        }
    }

    if (static_cast<int>(line.size()) > state.screencols) {
        line = line.substr(0, static_cast<size_t>(state.screencols));
    }
    ab.append(line);
    for (int i = static_cast<int>(line.size()); i < state.screencols; i++) ab.append(" ", 1);

    ab.append("\r\n", 2);
}

void ScreenRenderer::drawRows(AppendBuffer& ab, const ViewState& state) {
    const auto& ob = state.bufferManager.active();
    const model::Buffer& buffer = ob.buffer;
    int textRows = textAreaScreenRows(state);
    int gutter = gutterWidth(state);
    int textCols = std::max(state.screencols - gutter, 1);

    bool trueColor = state.config.trueColor;
    const io::ThemeColors& theme = state.config.theme;

    model::Position selStart{}, selEnd{};
    bool hasSelection = ob.selection.isActive && !ob.selection.empty();
    if (hasSelection) {
        selStart = ob.selection.start();
        selEnd = ob.selection.end();
    }

    for (int y = 0; y < textRows; y++) {
        int filerow = y + state.rowoff;

        if (gutter > 0) {
            ab.append(fgSequence(theme.lineNumberFg, trueColor));
            if (filerow < buffer.numRows()) {
                char numbuf[16];
                int n = snprintf(numbuf, sizeof(numbuf), "%*d ", gutter - 1, filerow + 1);
                ab.append(numbuf, static_cast<size_t>(n));
            } else {
                for (int k = 0; k < gutter; k++) ab.append(" ", 1);
            }
            ab.append(resetFgSequence());
        }

        if (filerow >= buffer.numRows()) {
            if (buffer.numRows() == 0 && y == textRows / 3) {
                char welcome[80];
                int welcomelen = snprintf(welcome, sizeof(welcome),
                                           "Cera editor -- version %s", "0.1.0");
                if (welcomelen > textCols) welcomelen = textCols;
                int padding = (textCols - welcomelen) / 2;
                if (padding) {
                    ab.append("~", 1);
                    padding--;
                }
                while (padding--) ab.append(" ", 1);
                ab.append(welcome, static_cast<size_t>(welcomelen));
            } else {
                ab.append("~", 1);
            }
        } else {
            const model::Row& row = buffer.row(filerow);
            const std::string& render = row.render();
            const auto& hl = row.hl();
            const auto& offsets = row.renderByteOffsets();

            int colWidthUsed = 0;
            int currentColorRgb = -1;
            bool inSelectionBg = false;

            for (int ci = state.coloff; ci < row.rsize() && colWidthUsed < textCols; ci++) {
                int charWidth = row.renderCharWidth(ci);
                if (colWidthUsed + charWidth > textCols) break;

                bool selected = false;
                if (hasSelection && filerow >= selStart.row && filerow <= selEnd.row) {
                    if (filerow == selStart.row && filerow == selEnd.row) {
                        selected = (ci >= selStart.col && ci < selEnd.col);
                    } else if (filerow == selStart.row) {
                        selected = (ci >= selStart.col);
                    } else if (filerow == selEnd.row) {
                        selected = (ci < selEnd.col);
                    } else {
                        selected = true;
                    }
                }

                if (selected != inSelectionBg) {
                    if (selected) {
                        ab.append(bgSequence(theme.statusBarBg, trueColor));
                    } else {
                        ab.append(resetBgSequence());
                    }
                    inSelectionBg = selected;
                }

                size_t byteOff = offsets[static_cast<size_t>(ci)];
                auto dc = model::utf8::decode(render, byteOff);

                if (dc.codepoint < 32 && dc.codepoint != 0) {
                    char sym = static_cast<char>('@' + dc.codepoint);
                    ab.append("\x1b[7m", 4);
                    ab.append(&sym, 1);
                    ab.append("\x1b[m", 3);
                    if (currentColorRgb != -1) {
                        ab.append(fgSequence(static_cast<uint32_t>(currentColorRgb), trueColor));
                    }
                } else {
                    model::Highlight h = hl[static_cast<size_t>(ci)];
                    uint32_t rgb = highlightToRgb(h, theme);
                    if (static_cast<int>(rgb) != currentColorRgb) {
                        currentColorRgb = static_cast<int>(rgb);
                        ab.append(fgSequence(rgb, trueColor));
                    }
                    ab.append(render.data() + byteOff, static_cast<size_t>(dc.byteLength));
                }

                colWidthUsed += charWidth;
            }

            ab.append(resetFgSequence());
            if (inSelectionBg) ab.append(resetBgSequence());
        }

        ab.append("\x1b[K", 3);
        ab.append("\r\n", 2);
    }
}

void ScreenRenderer::drawStatusBar(AppendBuffer& ab, const ViewState& state) {
    const auto& ob = state.bufferManager.active();
    const model::Buffer& buffer = ob.buffer;

    std::string left;
    if (state.search.active) {
        char buf[128];
        int n = snprintf(buf, sizeof(buf), " recherche: %s (%d/%d)",
                          state.search.term.c_str(), state.search.currentIndex,
                          state.search.totalMatches);
        left.assign(buf, static_cast<size_t>(n));
    } else {
        left = " ^H aide";
    }

    std::string filename = buffer.filename() ? *buffer.filename() : "[Sans nom]";
    size_t slash = filename.find_last_of('/');
    if (slash != std::string::npos) filename = filename.substr(slash + 1);
    std::string center = filename;
    if (buffer.dirty()) center += " [modifie]";

    int totalLines = std::max(buffer.numRows(), 1);
    char rightBuf[64];
    int rn = snprintf(rightBuf, sizeof(rightBuf), "%d/%d ", ob.cursor.cy + 1, totalLines);
    std::string right(rightBuf, static_cast<size_t>(rn));

    int total = state.screencols;
    int leftLen = static_cast<int>(left.size());
    int rightLen = static_cast<int>(right.size());
    int centerLen = static_cast<int>(center.size());

    if (leftLen > total) leftLen = total;
    ab.append(left.data(), static_cast<size_t>(leftLen));

    int usedByLeftAndRight = leftLen + rightLen;
    int remaining = total - usedByLeftAndRight;
    int centerStart = std::max((remaining - centerLen) / 2, 0);

    int written = leftLen;
    for (int i = 0; i < centerStart && written < total - rightLen; i++, written++) ab.append(" ", 1);

    int centerWrite = std::min(centerLen, std::max(total - rightLen - written, 0));
    ab.append(center.data(), static_cast<size_t>(centerWrite));
    written += centerWrite;

    while (written < total - rightLen) {
        ab.append(" ", 1);
        written++;
    }

    if (rightLen > 0 && total - written >= rightLen) {
        ab.append(right);
        written += rightLen;
    }
    while (written < total) {
        ab.append(" ", 1);
        written++;
    }

    ab.append("\r\n", 2);
}

void ScreenRenderer::drawHelpBar(AppendBuffer& ab, const ViewState& state) {
    if (!state.showHelpBar) return;

    std::string help = " " + shortHelpLine();
    if (static_cast<int>(help.size()) > state.screencols) {
        help = help.substr(0, static_cast<size_t>(state.screencols));
    }
    ab.append(help);
    for (int i = static_cast<int>(help.size()); i < state.screencols; i++) ab.append(" ", 1);

    ab.append("\r\n", 2);
}

void ScreenRenderer::drawMessageBar(AppendBuffer& ab, const ViewState& state) {
    ab.append("\x1b[K", 3);
    int msglen = static_cast<int>(state.statusMessage.size());
    if (msglen > state.screencols) msglen = state.screencols;
    if (msglen > 0 && state.statusMessageAgeSeconds < 5) {
        uint32_t color = state.config.theme.normal;
        if (state.statusMessageKind == MessageKind::Success) color = state.config.theme.successFg;
        else if (state.statusMessageKind == MessageKind::Error) color = state.config.theme.errorFg;

        ab.append(fgSequence(color, state.config.trueColor));
        ab.append(state.statusMessage.data(), static_cast<size_t>(msglen));
        ab.append(resetFgSequence());
    }
}

void ScreenRenderer::drawFullDoc(AppendBuffer& ab, const ViewState& state) {
    const io::ThemeColors& theme = state.config.theme;
    bool trueColor = state.config.trueColor;

    ab.append(fgSequence(theme.keyword1, trueColor));
    std::string title = " Documentation de Cera -- Echap pour revenir au fichier ";
    if (static_cast<int>(title.size()) > state.screencols) title = title.substr(0, static_cast<size_t>(state.screencols));
    ab.append(title);
    ab.append(resetFgSequence());
    ab.append("\x1b[K", 3);
    ab.append("\r\n", 2);

    int rowsUsed = 1;
    for (const auto& cat : fullDocumentation()) {
        if (rowsUsed >= state.screenrows - 1) break;
        ab.append(fgSequence(theme.keyword1, trueColor));
        std::string catLine = cat.name;
        ab.append(catLine);
        ab.append(resetFgSequence());
        ab.append("\x1b[K", 3);
        ab.append("\r\n", 2);
        rowsUsed++;

        for (const auto& entry : cat.entries) {
            if (rowsUsed >= state.screenrows - 1) break;
            char buf[256];
            int n = snprintf(buf, sizeof(buf), "  %-14s %s", entry.shortcut.c_str(), entry.description.c_str());
            std::string line(buf, static_cast<size_t>(n));
            if (static_cast<int>(line.size()) > state.screencols) {
                line = line.substr(0, static_cast<size_t>(state.screencols));
            }
            ab.append(line);
            ab.append("\x1b[K", 3);
            ab.append("\r\n", 2);
            rowsUsed++;
        }

        if (rowsUsed < state.screenrows - 1) {
            ab.append("\x1b[K", 3);
            ab.append("\r\n", 2);
            rowsUsed++;
        }
    }

    while (rowsUsed < state.screenrows) {
        ab.append("\x1b[K", 3);
        ab.append("\r\n", 2);
        rowsUsed++;
    }

    ab.append("\x1b[K", 3);
}

void ScreenRenderer::refresh(const ViewState& state) {
    AppendBuffer ab;

    ab.append("\x1b[?25l", 6);
    ab.append("\x1b[H", 3);

    if (state.overlay == OverlayMode::FullDoc) {
        drawFullDoc(ab, state);
        ab.append("\x1b[?25h", 6);
        term_.write(ab.data().data(), ab.data().size());
        return;
    }

    drawTabBar(ab, state);
    drawRows(ab, state);
    drawStatusBar(ab, state);
    drawHelpBar(ab, state);
    drawMessageBar(ab, state);

    const auto& ob = state.bufferManager.active();
    int gutter = gutterWidth(state);
    int tabBarHeight = (state.bufferManager.count() > 1) ? 1 : 0;

    char buf[32];
    int len = snprintf(buf, sizeof(buf), "\x1b[%d;%dH",
                        tabBarHeight + (ob.cursor.cy - state.rowoff) + 1,
                        gutter + (ob.cursor.rx - state.coloff) + 1);
    ab.append(buf, static_cast<size_t>(len));

    ab.append("\x1b[?25h", 6);

    term_.write(ab.data().data(), ab.data().size());
}

}
