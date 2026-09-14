#include "view/Colors.hpp"

namespace cera::view {

int highlightToColor(model::Highlight hl) {
    switch (hl) {
        case model::Highlight::Comment:
        case model::Highlight::MLComment:
            return 36;
        case model::Highlight::Keyword1:
            return 33;
        case model::Highlight::Keyword2:
            return 32;
        case model::Highlight::String:
            return 35;
        case model::Highlight::Number:
            return 31;
        case model::Highlight::Match:
            return 34;
        case model::Highlight::Normal:
        default:
            return 37;
    }
}

}
