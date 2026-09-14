#pragma once

#include <string>
#include <vector>

#include "model/Buffer.hpp"

namespace cera::io {

void openFile(model::Buffer& buffer, const std::string& filename);

long saveFile(model::Buffer& buffer, int* errnoOut);

}
