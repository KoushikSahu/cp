fun main() {
  var T = readln().toInt()
  while (T > 0) {
    solve()
    T -= 1
  }
}

fun solve() {
  val (n, m) = readln().split(" ").map { i -> i.toInt() }
  val a: MutableSet<Int> = readln().split(" ").map { i -> i.toInt() }.toMutableSet()
  val b: MutableSet<Int> = readln().split(" ").map { i -> i.toInt() }.toMutableSet()
  val cmn = a.intersect(b)
  val au = a.size - cmn.size
  val bu = b.size - cmn.size
  var ans = 0
  if (au > bu) {
    ans = 2 * (bu + 1)
  } else {
    ans = 2 * au + 1
  }
  println(ans)
}
