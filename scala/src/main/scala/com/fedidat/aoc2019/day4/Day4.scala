package com.fedidat.aoc2019.day4

object Day4 {

  def part1(range: Range.Inclusive): Int =
    range.count(n =>
      n.toString.toSeq.sorted.equals(n.toString.toSeq) &&
      n.toString.toSeq.sliding(2).toList.exists(s => s.head == s.last)
    )

  def restricted2Digits(d:Array[Int])(i:Int): Boolean =
    (i == 0 || d(i-1) < d(i)) && d(i) == d(i+1) && ((i == 4) || d(i+1) < d(i+2))

  def part2(range: Range.Inclusive): Int =
    range.count(n =>
      n.toString.toSeq.sorted.equals(n.toString.toSeq) &&
      (0 to 4).exists(restricted2Digits(n.toString.split("").map(_.toInt)))
    )

  def main(args: Array[String]): Unit = {
    val input = 193651 to 649729
    println(part1(input))
    println(part2(input))
  }
}