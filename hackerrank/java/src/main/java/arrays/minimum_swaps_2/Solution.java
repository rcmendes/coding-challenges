package arrays.minimum_swaps_2;

public class Solution {

    // Complete the minimumSwaps function below.
    static int minimumSwaps(int[] arr) {
        // check in sequence
        // swap with the original place owner
        // move forward

        int swaps = 0;
        for (int i = 0; i < arr.length; i++) {
            int expectedIndex = arr[i] - 1;
            if (i != expectedIndex) {
                int ownerIndex = findOwnerIndex(arr, i + 1, i + 1);
                swap(arr, i, ownerIndex);
                swaps++;
            }
        }

        return swaps;

    }

    static int findOwnerIndex(int[] array, int number, int startIndex) {
        for (int i = startIndex; i < array.length; i++) {
            if (array[i] == number) {
                return i;
            }
        }

        return -1;
    }

    static void swap(int[] array, int a, int b) {
        int temp = array[a];
        array[a] = array[b];
        array[b] = temp;
    }

    public static void main(String[] args) {
        int[] inputOk1 = new int[]{1, 3, 5, 2, 4, 6, 7};
        System.out.println(minimumSwaps(inputOk1));

        int[] inputOk2 = new int[]{7, 1, 3, 2, 4, 5, 6};
        System.out.println(minimumSwaps(inputOk2));

    }
}