import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.Arrays;

public class BenchMark {

    private static boolean sameContent(final String[] words1, final String[] words2) {
        Arrays.sort(words1);
        Arrays.sort(words2);
        return Arrays.equals(words1, words2);
    }

    public static void main(final String[] args) throws IOException {
        final BufferedReader bufferedReader = new BufferedReader(new FileReader("../input_2.txt"));

        final String[] words1 = bufferedReader.readLine().split(" ");
        final String[] words2 = bufferedReader.readLine().split(" ");
        bufferedReader.close();

        //System.out.println("words1 = " + Arrays.toString(words1) + ", size = " + words1.length);
        //System.out.println("words2 = " + Arrays.toString(words2) + ", size = " + words2.length);

        final boolean result1 = sameContent(words1, words2);

        System.out.println("JAVA");
        System.out.println("result1 = " + result1);
    }
}
