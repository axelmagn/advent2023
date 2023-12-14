from typing import Any, Generator, Optional
import unittest
import re
import sys


class Span(object):
    def __init__(self, row: int, col_start: int, col_end: int):
        self.row = row
        self.col_start = col_start
        self.col_end = col_end

    def overlaps(self, rhs: "Span") -> bool:
        return self.row == rhs.row and (
            (self.col_start <= rhs.col_start < self.col_end)
            or (self.col_start < rhs.col_end <= self.col_end)
            or (rhs.col_start <= self.col_start < rhs.col_end)
            or (rhs.col_start < self.col_end <= rhs.col_end)
        )

    def __eq__(self, rhs: Any) -> bool:
        if not isinstance(rhs, Span):
            return False
        return (
            self.row == rhs.row
            and self.col_start == rhs.col_start
            and self.col_end == rhs.col_end
        )

    def __str__(self):
        return "Span({}, [{},{}])".format(self.row, self.col_start, self.col_end)

    def __repr__(self):
        return self.__str__()

    def __hash__(self):
        return hash((self.row, self.col_start, self.col_end))


class Schematic(object):
    STAR_CHAR = "*"
    NUM_RE = re.compile(r"\d+")

    def __init__(self, lines: [str]):
        self.lines = lines
        pass

    def stars(self) -> Generator[Span, None, None]:
        for i, line in enumerate(self.lines):
            for j, c in enumerate(line):
                if c == Schematic.STAR_CHAR:
                    yield Span(i, j, j + 1)

    def match_nums(self, span: Span) -> Generator[Span, None, None]:
        """search the span for any numbers that overlap"""
        line = self.lines[span.row]
        line_nums = Schematic.NUM_RE.finditer(line)
        for num_match in line_nums:
            num_span = Span(span.row, num_match.start(), num_match.end())
            if span.overlaps(num_span):
                yield num_span

    def match_nums_in_square(
        self, row_start: int, row_end: int, col_start: int, col_end: int
    ) -> Generator[Span, None, None]:
        for i in range(row_start, row_end):
            span = Span(i, col_start, col_end)
            for num_match in self.match_nums(span):
                yield num_match

    def parse_span_num(self, span: Span) -> int:
        line = self.lines[span.row]
        term = line[span.col_start : span.col_end]
        return int(term)

    def gear_ratio(self, row: int, col: int) -> int:
        row_start = max(row - 1, 0)
        row_end = min(row + 2, len(self.lines))
        col_start = max(col - 1, 0)
        col_end = min(col + 2, len(self.lines[0]))
        nums = [
            self.parse_span_num(num_span)
            for num_span in self.match_nums_in_square(
                row_start, row_end, col_start, col_end
            )
        ]
        if len(nums) == 2:
            return nums[0] * nums[1]
        return 0

    def gear_ratio_sum(self):
        total = 0
        for star in self.stars():
            total += self.gear_ratio(star.row, star.col_start)
        return total


TEST_STRING = r"""467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""


class TestSpan(unittest.TestCase):
    def test_overlap(self):
        span1 = Span(0, 2, 5)
        span2 = Span(0, 4, 8)
        span3 = Span(0, 5, 8)

        self.assertTrue(span1.overlaps(span2))
        self.assertTrue(span2.overlaps(span1))

        self.assertFalse(span1.overlaps(span3))
        self.assertFalse(span3.overlaps(span1))

        pass


class TestSchematic(unittest.TestCase):
    def setUp(self) -> None:
        self.schematic = Schematic(TEST_STRING.splitlines())

    def test_stars(self):
        stars = [star for star in self.schematic.stars()]
        self.assertEqual(len(stars), 3)
        self.assertEqual(stars[0], Span(1, 3, 4))
        self.assertEqual(stars[1], Span(4, 3, 4))
        self.assertEqual(stars[2], Span(8, 5, 6))

    def test_match_nums(self):
        # match a span exactly
        span = Span(4, 0, 3)
        nums = [num for num in self.schematic.match_nums(span)]
        self.assertEqual(len(nums), 1)
        self.assertEqual(nums[0], span)

        # match a partially overlapping span
        span = Span(4, 2, 3)
        nums = [num for num in self.schematic.match_nums(span)]
        self.assertEqual(len(nums), 1)
        self.assertEqual(nums[0], Span(4, 0, 3))

        # match two nums
        span = Span(2, 3, 7)
        nums = [num for num in self.schematic.match_nums(span)]
        self.assertEqual(len(nums), 2)
        self.assertEqual(nums[0], Span(2, 2, 4))
        self.assertEqual(nums[1], Span(2, 6, 9))

    def test_match_nums_in_square(self):
        row_start = 0
        row_end = 3
        col_start = 2
        col_end = 5
        nums = [
            num
            for num in self.schematic.match_nums_in_square(
                row_start, row_end, col_start, col_end
            )
        ]
        self.assertEqual(len(nums), 2)
        self.assertTrue(Span(0, 0, 3) in nums)
        self.assertTrue(Span(2, 2, 4) in nums)

    def test_gear_ratio_sum(self):
        total = self.schematic.gear_ratio_sum()
        self.assertEqual(total, 467835)


def main():
    schematic = Schematic([line for line in sys.stdin])
    print(schematic.gear_ratio_sum())


if __name__ == "__main__":
    main()
    # unittest.main()
