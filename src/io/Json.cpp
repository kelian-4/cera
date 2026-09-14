#include "io/Json.hpp"

#include <cctype>
#include <cstdlib>

namespace cera::io::json {

const Value* Value::get(const std::string& key) const {
    if (type != Type::Object) return nullptr;
    auto it = objectValue.find(key);
    if (it == objectValue.end()) return nullptr;
    return it->second.get();
}

std::string Value::asString(const std::string& fallback) const {
    return type == Type::String ? stringValue : fallback;
}

double Value::asNumber(double fallback) const {
    return type == Type::Number ? numberValue : fallback;
}

int Value::asInt(int fallback) const {
    return type == Type::Number ? static_cast<int>(numberValue) : fallback;
}

bool Value::asBool(bool fallback) const {
    return type == Type::Bool ? boolValue : fallback;
}

namespace {

class Parser {
public:
    explicit Parser(const std::string& text) : text_(text) {}

    ParseResult run() {
        skipWhitespace();
        ValuePtr v = parseValue();
        if (!v) return {nullptr, false, error_};
        skipWhitespace();
        if (pos_ != text_.size()) {
            return {nullptr, false, "caractères en trop après la valeur JSON"};
        }
        return {v, true, ""};
    }

private:
    const std::string& text_;
    size_t pos_ = 0;
    std::string error_;

    char peek() const { return pos_ < text_.size() ? text_[pos_] : '\0'; }
    char advance() { return pos_ < text_.size() ? text_[pos_++] : '\0'; }

    void skipWhitespace() {
        while (pos_ < text_.size() && std::isspace(static_cast<unsigned char>(text_[pos_]))) pos_++;
    }

    bool expect(char c) {
        if (peek() != c) {
            error_ = std::string("attendu '") + c + "'";
            return false;
        }
        pos_++;
        return true;
    }

    ValuePtr parseValue() {
        skipWhitespace();
        char c = peek();
        if (c == '{') return parseObject();
        if (c == '[') return parseArray();
        if (c == '"') return parseString();
        if (c == 't' || c == 'f') return parseBool();
        if (c == 'n') return parseNull();
        if (c == '-' || std::isdigit(static_cast<unsigned char>(c))) return parseNumber();
        error_ = "valeur JSON inattendue";
        return nullptr;
    }

    ValuePtr parseObject() {
        auto v = std::make_shared<Value>();
        v->type = Type::Object;
        if (!expect('{')) return nullptr;
        skipWhitespace();
        if (peek() == '}') {
            pos_++;
            return v;
        }
        while (true) {
            skipWhitespace();
            if (peek() != '"') {
                error_ = "clé attendue dans l'objet";
                return nullptr;
            }
            ValuePtr keyVal = parseString();
            if (!keyVal) return nullptr;
            skipWhitespace();
            if (!expect(':')) return nullptr;
            ValuePtr val = parseValue();
            if (!val) return nullptr;
            v->objectValue[keyVal->stringValue] = val;
            skipWhitespace();
            char c = advance();
            if (c == ',') continue;
            if (c == '}') break;
            error_ = "',' ou '}' attendu";
            return nullptr;
        }
        return v;
    }

    ValuePtr parseArray() {
        auto v = std::make_shared<Value>();
        v->type = Type::Array;
        if (!expect('[')) return nullptr;
        skipWhitespace();
        if (peek() == ']') {
            pos_++;
            return v;
        }
        while (true) {
            ValuePtr val = parseValue();
            if (!val) return nullptr;
            v->arrayValue.push_back(val);
            skipWhitespace();
            char c = advance();
            if (c == ',') continue;
            if (c == ']') break;
            error_ = "',' ou ']' attendu";
            return nullptr;
        }
        return v;
    }

    ValuePtr parseString() {
        auto v = std::make_shared<Value>();
        v->type = Type::String;
        if (!expect('"')) return nullptr;
        std::string out;
        while (true) {
            if (pos_ >= text_.size()) {
                error_ = "chaîne non terminée";
                return nullptr;
            }
            char c = advance();
            if (c == '"') break;
            if (c == '\\') {
                char esc = advance();
                switch (esc) {
                    case '"': out.push_back('"'); break;
                    case '\\': out.push_back('\\'); break;
                    case '/': out.push_back('/'); break;
                    case 'n': out.push_back('\n'); break;
                    case 't': out.push_back('\t'); break;
                    case 'r': out.push_back('\r'); break;
                    case 'b': out.push_back('\b'); break;
                    case 'f': out.push_back('\f'); break;
                    case 'u': {
                        if (pos_ + 4 > text_.size()) {
                            error_ = "séquence \\u incomplète";
                            return nullptr;
                        }
                        std::string hex = text_.substr(pos_, 4);
                        pos_ += 4;
                        unsigned int cp = static_cast<unsigned int>(std::strtoul(hex.c_str(), nullptr, 16));
                        if (cp <= 0x7F) {
                            out.push_back(static_cast<char>(cp));
                        } else if (cp <= 0x7FF) {
                            out.push_back(static_cast<char>(0xC0 | (cp >> 6)));
                            out.push_back(static_cast<char>(0x80 | (cp & 0x3F)));
                        } else {
                            out.push_back(static_cast<char>(0xE0 | (cp >> 12)));
                            out.push_back(static_cast<char>(0x80 | ((cp >> 6) & 0x3F)));
                            out.push_back(static_cast<char>(0x80 | (cp & 0x3F)));
                        }
                        break;
                    }
                    default:
                        error_ = "séquence d'échappement invalide";
                        return nullptr;
                }
            } else {
                out.push_back(c);
            }
        }
        v->stringValue = out;
        return v;
    }

    ValuePtr parseBool() {
        auto v = std::make_shared<Value>();
        v->type = Type::Bool;
        if (text_.compare(pos_, 4, "true") == 0) {
            v->boolValue = true;
            pos_ += 4;
            return v;
        }
        if (text_.compare(pos_, 5, "false") == 0) {
            v->boolValue = false;
            pos_ += 5;
            return v;
        }
        error_ = "littéral booléen invalide";
        return nullptr;
    }

    ValuePtr parseNull() {
        if (text_.compare(pos_, 4, "null") == 0) {
            pos_ += 4;
            auto v = std::make_shared<Value>();
            v->type = Type::Null;
            return v;
        }
        error_ = "littéral 'null' invalide";
        return nullptr;
    }

    ValuePtr parseNumber() {
        size_t start = pos_;
        if (peek() == '-') pos_++;
        while (std::isdigit(static_cast<unsigned char>(peek()))) pos_++;
        if (peek() == '.') {
            pos_++;
            while (std::isdigit(static_cast<unsigned char>(peek()))) pos_++;
        }
        if (peek() == 'e' || peek() == 'E') {
            pos_++;
            if (peek() == '+' || peek() == '-') pos_++;
            while (std::isdigit(static_cast<unsigned char>(peek()))) pos_++;
        }
        std::string numStr = text_.substr(start, pos_ - start);
        auto v = std::make_shared<Value>();
        v->type = Type::Number;
        v->numberValue = std::strtod(numStr.c_str(), nullptr);
        return v;
    }
};

}

ParseResult parse(const std::string& text) {
    Parser p(text);
    return p.run();
}

}
