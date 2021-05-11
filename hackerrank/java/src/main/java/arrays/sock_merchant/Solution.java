package arrays.sock_merchant;

import java.io.*;
import java.util.*;

public class Solution {

    // Complete the sockMerchant function below.
    static int sockMerchant(int n, int[] ar) {
        HashMap<Integer, Integer> numberOfSockets = new HashMap<Integer, Integer>();
        for (int socket : ar) {
            Integer count = numberOfSockets.getOrDefault(socket, 0);
            numberOfSockets.put(socket, ++count);
        }

        return numberOfSockets.values().stream().mapToInt(count -> Math.floorDiv(count, 2)).sum();
    }
}
