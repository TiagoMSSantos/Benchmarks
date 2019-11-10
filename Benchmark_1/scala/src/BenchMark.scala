import java.io.{BufferedReader, FileReader, IOException}

import scala.util.Sorting

object BenchMark {
  private def sameContent(words1: Array[String], words2: Array[String]) = {
    Sorting.quickSort(words1)
    Sorting.quickSort(words2)
    words1.sameElements(words2)
  }

  @throws[IOException]
  def main(args: Array[String]): Unit = {
    val bufferedReader = new BufferedReader(new FileReader("../input_2.txt"))
    val words1 : Array[String] = bufferedReader.readLine.split(" ")
    val words2 : Array[String] = bufferedReader.readLine.split(" ")
    bufferedReader.close()
    System.out.println("words1 = " + words1.mkString(" ") + ", size = " + words1.length)
    System.out.println("words2 = " + words2.mkString(" ") + ", size = " + words2.length)
    val start = System.nanoTime
    val result1 = sameContent(words1, words2)
    val end = System.nanoTime
    val timeInMs = (end - start) / 1000000
    System.out.println("result1 = " + result1 + ", time = " + timeInMs + "ms")
  }
}
