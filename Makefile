CXX ?= g++
CXXFLAGS ?= -std=c++17 -Wall -Wextra -Isrc
LDFLAGS ?=

CORE_SRCS := \
	src/terminal/Terminal.cpp \
	src/model/Utf8.cpp \
	src/model/Row.cpp \
	src/model/Buffer.cpp \
	src/syntax/Syntax.cpp \
	src/syntax/CSyntax.cpp \
	src/syntax/PythonSyntax.cpp \
	src/syntax/JavaScriptSyntax.cpp \
	src/syntax/MarkdownSyntax.cpp \
	src/io/Json.cpp \
	src/io/Config.cpp \
	src/io/FileIO.cpp \
	src/view/Colors.cpp \
	src/view/HelpText.cpp \
	src/view/ScreenRenderer.cpp \
	src/editor/UndoStack.cpp \
	src/editor/Clipboard.cpp \
	src/editor/BufferManager.cpp \
	src/editor/Editor.cpp

CORE_OBJS := $(CORE_SRCS:.cpp=.o)

.PHONY: all clean test werror

all: cera

cera: src/main.o $(CORE_OBJS)
	$(CXX) $(CXXFLAGS) $^ -o $@ $(LDFLAGS)

model_tests: tests/model_tests.o $(CORE_OBJS)
	$(CXX) $(CXXFLAGS) $^ -o $@ $(LDFLAGS)

test: model_tests
	./model_tests

werror:
	$(MAKE) clean
	$(MAKE) all CXXFLAGS="$(CXXFLAGS) -Werror"
	$(MAKE) model_tests CXXFLAGS="$(CXXFLAGS) -Werror"

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f cera model_tests
	find src tests -name '*.o' -delete
