#include "view/ScreenRenderer.hpp"

#include <cctype>
#include <cstdio>

#include "view/Colors.hpp"

namespace cera::view {

void ScreenRenderer::drawRows(AppendBuffer& ab, const ViewState& state) {
    const model::Buffer& buffer = state.buffer;

    for (int y = 0; y < state.screenrows; y++) {
        int filerow = y + state.rowoff;
        if (filerow >= buffer.numRows()) {
            if (buffer.numRows() == 0 && y == state.screenrows / 3) {
                char welcome[80];
                int welcomelen = snprintf(welcome, sizeof(welcome),
                                           "Cera editor -- version %s", "0.0.1");
                if (welcomelen > state.screencols) welcomelen = state.screencols;
                int padding = (state.screencols - welcomelen) / 2;
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
            int len = row.rsize() - state.coloff;
            if (len < 0) len = 0;
            if (len > state.screencols) len = state.screencols;

            const std::string& render = row.render();
            const auto& hl = row.hl();
            int currentColor = -1;

            for (int j = 0; j < len; j++) {
                int idx = state.coloff + j;
                unsigned char ch = static_cast<unsigned char>(render[static_cast<size_t>(idx)]);

                if (std::iscntrl(ch)) {
                    char sym = (ch <= 26) ? static_cast<char>('@' + ch) : '?';
                    ab.append("\x1b[7m", 4);
                    ab.append(&sym, 1);
                    ab.append("\x1b[m", 3);
                    if (currentColor != -1) {
                        char buf[16];
                        int clen = snprintf(buf, sizeof(buf), "\x1b[%dm", currentColor);
                        ab.append(buf, static_cast<size_t>(clen));
                    }
                }

                model::Highlight h = hl[static_cast<size_t>(idx)];
                if (h == model::Highlight::Normal) {
                    if (currentColor != -1) {
                        ab.append("\x1b[39m", 5);
                        currentColor = -1;
                    }
                    ab.append(&render[static_cast<size_t>(idx)], 1);
                } else {
                    int color = highlightToColor(h);
                    if (color != currentColor) {
                        currentColor = color;
                        char buf[16];
                        int clen = snprintf(buf, sizeof(buf), "\x1b[%dm", color);
                        ab.append(buf, static_cast<size_t>(clen));
                    }
                    ab.append(&render[static_cast<size_t>(idx)], 1);
                }
            }
            ab.append("\x1b[39m", 5);
        }

        ab.append("\x1b[K", 3);
        ab.append("\r\n", 2);
    }
}

void ScreenRenderer::drawStatusBar(AppendBuffer& ab, const ViewState& state) {
    ab.append("\x1b[7m", 4);

    const model::Buffer& buffer = state.buffer;
    std::string filename = buffer.filename() ? *buffer.filename() : "[No Name]";

    char status[80];
    char rstatus[80];
    int len = snprintf(status, sizeof(status), "%.20s - %d lines %s",
                        filename.c_str(), buffer.numRows(),
                        buffer.dirty() ? "(modified)" : "");
    int rlen = snprintf(rstatus, sizeof(rstatus), "%s | %d/%d",
                         state.activeSyntax ? state.activeSyntax->filetype.c_str() : "no ft",
                         state.cursor.cy + 1, buffer.numRows());

    if (len > state.screencols) len = state.screencols;
    ab.append(status, static_cast<size_t>(len));

    while (len < state.screencols) {
        if (state.screencols - len == rlen) {
            ab.append(rstatus, static_cast<size_t>(rlen));
            break;
        }
        ab.append(" ", 1);
        len++;
    }

    ab.append("\x1b[m", 3);
    ab.append("\r\n", 2);
}

void ScreenRenderer::drawMessageBar(AppendBuffer& ab, const ViewState& state) {
    ab.append("\x1b[K", 3);
    int msglen = static_cast<int>(state.statusMessage.size());
    if (msglen > state.screencols) msglen = state.screencols;
    if (msglen > 0 && state.statusMessageAgeSeconds < 5) {
        ab.append(state.statusMessage.data(), static_cast<size_t>(msglen));
    }
}

void ScreenRenderer::refresh(const ViewState& state) {
    AppendBuffer ab;

    ab.append("\x1b[?25l", 6);
    ab.append("\x1b[H", 3);

    drawRows(ab, state);
    drawStatusBar(ab, state);
    drawMessageBar(ab, state);

    char buf[32];
    int rx = state.cursor.rx;
    int len = snprintf(buf, sizeof(buf), "\x1b[%d;%dH",
                        (state.cursor.cy - state.rowoff) + 1,
                        (rx - state.coloff) + 1);
    ab.append(buf, static_cast<size_t>(len));

    ab.append("\x1b[?25h", 6);

    term_.write(ab.data().data(), ab.data().size());
}

}
