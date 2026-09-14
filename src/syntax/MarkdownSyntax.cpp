#include "syntax/Syntax.hpp"

namespace cera::syntax {

const Syntax& markdownSyntax() {
    static const Syntax kMdSyntax{
        "markdown",
        {".md", ".markdown"},
        {},
        "",
        "",
        "",
        0,
    };
    return kMdSyntax;
}

}
