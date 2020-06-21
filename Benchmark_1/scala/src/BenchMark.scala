import java.io.{BufferedReader, FileReader, IOException}

object BenchMark {
  private def sameContent(words1: String, words2: String) = {
    val line1 : Array[Char] = words1.toCharArray()
    val line2 : Array[Char] = words2.toCharArray()
    java.util.Arrays.parallelSort(line1)
    java.util.Arrays.parallelSort(line2)
    java.util.Arrays.equals(line1, line2)
  }

  @throws[IOException]
  def main(args: Array[String]): Unit = {
    val bufferedReader = new BufferedReader(new FileReader("../input_2.txt"))
    val words1 : String = bufferedReader.readLine
    val words2 : String = bufferedReader.readLine
    bufferedReader.close()
    //System.out.println("words1 = " + words1.mkString(" ") + ", size = " + words1.length)
    //System.out.println("words2 = " + words2.mkString(" ") + ", size = " + words2.length)
    val result1 = sameContent(words1, words2)
    System.out.println("SCALA")
    System.out.println("result1 = " + result1)
  }
}
