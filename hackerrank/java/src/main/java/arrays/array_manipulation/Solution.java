package arrays.array_manipulation;

//TODO Incomplete due to performance (time limit)
public class Solution {

    static long arrayManipulation(int n, int[][] queries) {
        long max = 0;
        long[] array = new long[n];
        for (int[] query : queries) {
            int fromIndex = query[0];
            int toIndex = query[1];
            int value = query[2];

            for (int i = fromIndex; i <= toIndex; i++) {
                long newValue = array[i-1] + value;
                array[i-1] = newValue;
                if (max < newValue) {
                    max = newValue;
                }
            }
        }

        return max;

    }

    public static void main(String[] args) {

    }
}
