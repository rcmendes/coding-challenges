package arrays.left_rotation;

import java.io.IOException;

public class Solution {

    // Complete the rotLeft function below.
    static int[] rotLeft(int[] a, int d) {
        int shifts = d % a.length;

        int[] result = new int[a.length];

        for (int i = 0; i < a.length; i++) {
            if (i < shifts) {
                result[a.length - (shifts - i)] = a[i];
            } else {
                result[i - shifts] = a[i];
            }
        }

        return result;
    }

    public static void main(String[] args) throws IOException {
        int[] input1 = new int[] { 1, 2, 3, 4, 5 };
        int[] input2 = new int[] { 41, 73, 89, 7, 10, 1, 59, 58, 84, 77, 77, 97, 58, 1, 86, 58, 26, 10, 86, 51 };

        int[] r = rotLeft(input1, 4);
        for (int v : r) {
            System.out.print(v + " ");
        }

        System.out.println("\n----");

        r = rotLeft(input1, 2);
        for (int v : r) {
            System.out.print(v + " ");
        }

        System.out.println("\n----");
        r = rotLeft(input2, 10);
        for (int v : r) {
            System.out.print(v + " ");
        }
    }
}
