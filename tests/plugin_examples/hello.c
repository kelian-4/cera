#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char *NAME = "c";

static long find_number(const char *line, const char *key) {
    char pattern[64];
    snprintf(pattern, sizeof pattern, "\"%s\":", key);
    const char *at = strstr(line, pattern);
    if (!at) {
        return -1;
    }
    return strtol(at + strlen(pattern), NULL, 10);
}

static int find_string(const char *line, const char *key, char *out, size_t capacity) {
    char pattern[64];
    snprintf(pattern, sizeof pattern, "\"%s\":\"", key);
    const char *at = strstr(line, pattern);
    if (!at) {
        return 0;
    }
    at += strlen(pattern);
    const char *end = strchr(at, '"');
    if (!end) {
        return 0;
    }
    size_t length = (size_t)(end - at);
    if (length >= capacity) {
        length = capacity - 1;
    }
    memcpy(out, at, length);
    out[length] = '\0';
    return 1;
}

static void render(const char *text) {
    printf("{\"type\":\"render\",\"lines\":[{\"segments\":[{\"text\":\"%s\",\"color\":\"#82C0FF\",\"bold\":true}]}]}\n", text);
    fflush(stdout);
}

static void handle_key(const char *key) {
    char text[128];
    if (strcmp(key, "q") == 0) {
        printf("{\"type\":\"close\"}\n");
    } else if (strcmp(key, "o") == 0) {
        printf("{\"type\":\"open_file\",\"path\":\"/tmp/cera-plugin-test.txt\"}\n");
    } else if (strcmp(key, "s") == 0) {
        printf("{\"type\":\"status\",\"message\":\"status %s\",\"kind\":\"error\"}\n", NAME);
    } else {
        snprintf(text, sizeof text, "key %s %s", NAME, key);
        render(text);
        return;
    }
    fflush(stdout);
}

int main(void) {
    char line[4096];
    char type[32];
    char mode[32] = "fullscreen";
    char key[32];
    char text[128];

    printf("debug: starting %s\n", NAME);
    fflush(stdout);

    while (fgets(line, sizeof line, stdin)) {
        if (!find_string(line, "type", type, sizeof type)) {
            continue;
        }
        if (strcmp(type, "init") == 0) {
            find_string(line, "mode", mode, sizeof mode);
            snprintf(text, sizeof text, "hello %s %s %ldx%ld", NAME, mode,
                     find_number(line, "screen_rows"), find_number(line, "screen_cols"));
            render(text);
        } else if (strcmp(type, "resize") == 0) {
            snprintf(text, sizeof text, "resize %s %ldx%ld", NAME,
                     find_number(line, "screen_rows"), find_number(line, "screen_cols"));
            render(text);
        } else if (strcmp(type, "key") == 0) {
            if (find_string(line, "key", key, sizeof key)) {
                handle_key(key);
            }
        } else if (strcmp(type, "click") == 0) {
            snprintf(text, sizeof text, "click %s %ld", NAME, find_number(line, "row"));
            render(text);
        } else if (strcmp(type, "focus") == 0) {
            snprintf(text, sizeof text, "focus %s %s", NAME,
                     strstr(line, "\"focused\":true") ? "true" : "false");
            render(text);
        } else if (strcmp(type, "shutdown") == 0) {
            return 0;
        }
    }
    return 0;
}
