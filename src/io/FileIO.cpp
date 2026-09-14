#include "io/FileIO.hpp"

#include <cerrno>
#include <cstdio>
#include <fcntl.h>
#include <unistd.h>

namespace cera::io {

void openFile(model::Buffer& buffer, const std::string& filename) {
    buffer.setFilename(filename);

    FILE* fp = fopen(filename.c_str(), "r");
    if (!fp) {
        buffer.setLines({});
        return;
    }

    std::vector<std::string> lines;
    char* line = nullptr;
    size_t linecap = 0;
    ssize_t linelen;
    while ((linelen = getline(&line, &linecap, fp)) != -1) {
        while (linelen > 0 &&
               (line[linelen - 1] == '\n' || line[linelen - 1] == '\r')) {
            linelen--;
        }
        lines.emplace_back(line, static_cast<size_t>(linelen));
    }
    free(line);
    fclose(fp);

    buffer.setLines(lines);
}

long saveFile(model::Buffer& buffer, int* errnoOut) {
    if (!buffer.filename()) {
        if (errnoOut) *errnoOut = EINVAL;
        return -1;
    }

    std::string content = buffer.toString();

    int fd = open(buffer.filename()->c_str(), O_RDWR | O_CREAT, 0644);
    if (fd == -1) {
        if (errnoOut) *errnoOut = errno;
        return -1;
    }

    long len = static_cast<long>(content.size());
    if (ftruncate(fd, len) == -1) {
        if (errnoOut) *errnoOut = errno;
        close(fd);
        return -1;
    }

    ssize_t written = write(fd, content.data(), content.size());
    close(fd);

    if (written != static_cast<ssize_t>(content.size())) {
        if (errnoOut) *errnoOut = errno;
        return -1;
    }

    buffer.clearDirty();
    return len;
}

}
