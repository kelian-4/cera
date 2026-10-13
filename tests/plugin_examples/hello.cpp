#include <iostream>
#include <string>

static const std::string NAME = "cpp";

static long find_number(const std::string &line, const std::string &key) {
    const std::string pattern = "\"" + key + "\":";
    const auto at = line.find(pattern);
    if (at == std::string::npos) {
        return -1;
    }
    return std::stol(line.substr(at + pattern.size()));
}

static bool find_string(const std::string &line, const std::string &key, std::string &out) {
    const std::string pattern = "\"" + key + "\":\"";
    const auto at = line.find(pattern);
    if (at == std::string::npos) {
        return false;
    }
    const auto start = at + pattern.size();
    const auto end = line.find('"', start);
    if (end == std::string::npos) {
        return false;
    }
    out = line.substr(start, end - start);
    return true;
}

static void render(const std::string &text) {
    std::cout << "{\"type\":\"render\",\"lines\":[{\"segments\":[{\"text\":\"" << text
              << "\",\"color\":\"#82C0FF\",\"bold\":true}]}]}" << std::endl;
}

static void handle_key(const std::string &key) {
    if (key == "q") {
        std::cout << "{\"type\":\"close\"}" << std::endl;
    } else if (key == "o") {
        std::cout << "{\"type\":\"open_file\",\"path\":\"/tmp/cera-plugin-test.txt\"}" << std::endl;
    } else if (key == "s") {
        std::cout << "{\"type\":\"status\",\"message\":\"status " << NAME << "\",\"kind\":\"error\"}"
                  << std::endl;
    } else {
        render("key " + NAME + " " + key);
    }
}

int main() {
    std::string mode = "fullscreen";
    std::cout << "debug: starting " << NAME << std::endl;

    std::string line;
    while (std::getline(std::cin, line)) {
        std::string type;
        if (!find_string(line, "type", type)) {
            continue;
        }
        if (type == "init") {
            find_string(line, "mode", mode);
            render("hello " + NAME + " " + mode + " " + std::to_string(find_number(line, "screen_rows")) +
                   "x" + std::to_string(find_number(line, "screen_cols")));
        } else if (type == "resize") {
            render("resize " + NAME + " " + std::to_string(find_number(line, "screen_rows")) + "x" +
                   std::to_string(find_number(line, "screen_cols")));
        } else if (type == "key") {
            std::string key;
            if (find_string(line, "key", key)) {
                handle_key(key);
            }
        } else if (type == "click") {
            render("click " + NAME + " " + std::to_string(find_number(line, "row")));
        } else if (type == "focus") {
            const bool focused = line.find("\"focused\":true") != std::string::npos;
            render("focus " + NAME + " " + (focused ? "true" : "false"));
        } else if (type == "shutdown") {
            return 0;
        }
    }
    return 0;
}
