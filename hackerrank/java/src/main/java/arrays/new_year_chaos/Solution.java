package arrays.new_year_chaos;

public class Solution {
    // Complete the minimumBribes function below.
    static void minimumBribes(int[] q) {
        int[] base = generate(q.length);

        boolean tooChaotic = false;
        int bribes = 0;
        for (int i = 0; i < q.length; i++) {
            if (base[i] != q[i]) {
                // calculate shifts
                int number = q[i];
                int currentIndex = findIndex(base, number, i + 1);
                int shifts = currentIndex - i;

                // if shifts > 2, too chaotic and skip
                if (shifts > 2) {
                    tooChaotic = true;
                    break;
                }

                // bribe according to the shifts
                // increment bribes counter with shifts
                if (shifts > 0 && shifts <= 2) {
                    bribe(base, currentIndex, shifts);
                    bribes += shifts;
                }
            }
        }

        if (tooChaotic) {
            System.out.println("Too chaotic");
        } else {

            System.out.println(bribes);
        }
    }

    static int findIndex(int[] array, int number, int startIndex) {
        for (int i = startIndex; i < array.length; i++) {
            if (array[i] == number) {
                return i;
            }
        }

        return -1;
    }

    static void bribe(int[] array, int index, int shifts) {
        for (int s = 0; s < shifts; s++) {
            int temp = array[index - 1];
            array[index - 1] = array[index];
            array[index] = temp;
            index --;
        }
    }

    static int[] generate(int length) {
        int[] array = new int[length];
        for (int i = 0; i < length; i++) {
            array[i] = i + 1;
        }

        return array;
    }


    public static void main(String[] args) {
        int[] inputOk1 = new int[]{1, 2, 3, 4, 5};
        int[] inputOk2 = new int[]{2, 1, 5, 3, 4};
        int[] inputFail = new int[]{2, 5, 1, 3, 4};
        int[] inputFail2 = new int[]{1, 2, 5, 3, 7, 8, 6, 4};
//        minimumBribes(inputOk1);
//        minimumBribes(inputOk2);
//        minimumBribes(inputFail);
        minimumBribes(inputFail2);
    }
}