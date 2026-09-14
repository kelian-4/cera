#pragma once

#include <map>
#include <memory>
#include <string>
#include <variant>
#include <vector>

namespace cera::io::json {

class Value;
using ValuePtr = std::shared_ptr<Value>;

enum class Type { Null, Bool, Number, String, Array, Object };

class Value {
public:
    Type type = Type::Null;
    bool boolValue = false;
    double numberValue = 0.0;
    std::string stringValue;
    std::vector<ValuePtr> arrayValue;
    std::map<std::string, ValuePtr> objectValue;

    bool isObject() const { return type == Type::Object; }
    bool isString() const { return type == Type::String; }
    bool isNumber() const { return type == Type::Number; }
    bool isBool() const { return type == Type::Bool; }

    const Value* get(const std::string& key) const;

    std::string asString(const std::string& fallback = "") const;
    double asNumber(double fallback = 0.0) const;
    int asInt(int fallback = 0) const;
    bool asBool(bool fallback = false) const;
};

struct ParseResult {
    ValuePtr value;
    bool ok = false;
    std::string error;
};

ParseResult parse(const std::string& text);

}
