#include "editor/Editor.hpp"

int main(int argc, char* argv[]) {
    cera::editor::Editor editor;

    for (int i = 1; i < argc; i++) {
        editor.openFile(argv[i]);
    }

    editor.run();
}
