#pragma once

#include <string>
#include <vector>

namespace cera::view {

struct HelpEntry {
    std::string shortcut;
    std::string description;
};

struct HelpCategory {
    std::string name;
    std::vector<HelpEntry> entries;
};

std::string shortHelpLine();

const std::vector<HelpCategory>& fullDocumentation();

}
