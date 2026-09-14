#include "editor/Editor.hpp"

int main(int argc, char* argv[]) {
    cera::editor::Editor editor;

    if (argc >= 2) {
        editor.openFile(argv[1]);
    }

    editor.run();
}
