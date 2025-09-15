fun main(args: Array<String>) {
  var T = readln().toInt()
  while (T > 0) {
    solve()
    T -= 1
  }
}

fun solve() {
  val n = readln().toInt()
  val a = readln().split(" ").map { i -> i.toInt() }
  var nvr = Array<Boolean>(a.size) { true }
  for(i in 1..100) {
    for(j in 0.until(a.size)) {
      if(i >= a[j]) {
        nvr[j] = false
        break
      }
    }
  }
  var ans = mutableListOf<Int>()
  for(i in 0.until(a.size)) {
    if(nvr[i] == true) {
      ans.add(i + 1)
    }
  }
  println(ans.size)
  for(i in 0.until(ans.size)) {
    print(ans[i])
    print(" ")
  }
  println()
}
