#pragma once

namespace cera::model {

struct Position {
    int row = 0;
    int col = 0;
};

inline bool operator==(const Position& a, const Position& b) {
    return a.row == b.row && a.col == b.col;
}

inline bool positionLess(const Position& a, const Position& b) {
    if (a.row != b.row) return a.row < b.row;
    return a.col < b.col;
}

struct Selection {
    Position anchor;
    Position active;
    bool isActive = false;

    Position start() const { return positionLess(anchor, active) ? anchor : active; }
    Position end() const { return positionLess(anchor, active) ? active : anchor; }

    bool empty() const { return anchor == active; }
};

}
