import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStreamReader;

public class RecherchePLSSC {

    private static String PLSSC_REC(String S1, String S2, int i, int j) {
        // System.out.println("S1:" + S1 + " S2: " + S2 + " i:j " + i + ":" + j);

        if (S1.length() <= i || S2.length() <= j) {
            return "";
        }

        char xA = S1.charAt(i);
        char yB = S2.charAt(j);

        if (xA == yB) {
            return xA + PLSSC_REC(S1, S2, i + 1, j + 1);
        } else {
            String case1 = PLSSC_REC(S1, S2, i + 1, j);
            String case2 = PLSSC_REC(S1, S2, i, j + 1);
            if (case1.length() >= case2.length()) {
                return case1;
            } else {
                return case2;
            }
        }
    }

    // Recherche d'une PLSSC de 2 chaînes, naïf
    static String PLSSC(String S1, String S2) {
        return PLSSC_REC(S1, S2, 0, 0);
    }

    private final static String GET_IN_CACHE(String[][] cache, String S1, String S2, int i, int j) {
        if (S1.length() <= i || S2.length() <= j) {
            return "";
        }

        if (cache[i][j] != null) {
            return cache[i][j];
        } else {
            String plssc = PLSSC_PD_REC(cache, S1, S2, i, j);
            cache[i][j] = plssc;
            return plssc;
        }
    }

    private static String PLSSC_PD_REC(String[][] cache, String S1, String S2, int i, int j) {
        // System.out.println("S1:" + S1 + " S2: " + S2 + " i:j " + i + ":" + j);

        if (S1.length() <= i || S2.length() <= j) {
            return "";
        }

        char xA = S1.charAt(i);
        char yB = S2.charAt(j);

        if (xA == yB) {
            return xA + GET_IN_CACHE(cache, S1, S2, i + 1, j + 1);
        } else {
            String case1 = GET_IN_CACHE(cache, S1, S2, i + 1, j);
            String case2 = GET_IN_CACHE(cache, S1, S2, i, j + 1);
            if (case1.length() >= case2.length()) {
                return case1;
            } else {
                return case2;
            }
        }
    }

    // Recherche d'une PLSSC de 2 chaînes, prog. dyn.
    static String PLSSC_PD_VERSION_REC(String S1, String S2) {

        String[][] cache = new String[S1.length()][S2.length()];
        // Initialise toutes les cases à null
        for (int i = 0; i < S1.length(); i++) {
            for (int j = 0; j < S2.length(); j++) {
                cache[i][j] = null;
            }
        }

        return PLSSC_PD_REC(cache, S1, S2, 0, 0);
    }


    public static void printTab(int[][] tab) {
        for (int j = 0; j < tab[0].length; j++) {
            for (int i = 0; i < tab.length; i++) {
                System.out.print(tab[i][j] + " ");
            }
            System.out.println();
        }
    }















    
    // version iterative
    static String PLSSC_PD(String S1, String S2) {

        int sizeS1 = S1.length() + 1;
        int sizeS2 = S2.length() + 1;
        int tab[][] = new int[sizeS1][sizeS2];

        for (int w = 0; w < sizeS1; w++) {
            for (int v = 0; v < sizeS2; v++) {
                if (w == 0 || v == 0) {
                    tab[w][v] = 0;
                } else if (S1.charAt(w - 1) == S2.charAt(v - 1)) {
                    tab[w][v] = tab[w - 1][v - 1] + 1;
                } else {
                    tab[w][v] = Math.max(tab[w - 1][v], tab[w][v - 1]);
                }

                // System.out.println(w + ":" + v);
                // printTab(tab);
                // System.out.println();
            }
        }

        // printTab(tab);

        // Construct the result
        StringBuilder result = new StringBuilder();
        int i = sizeS1 - 1;
        int j = sizeS2 - 1;

        while (i > 0 && j > 0) {
            
            if (S1.charAt(i - 1) == S2.charAt(j - 1)) {
                result.insert(0, S1.charAt(i - 1));
                i--;
                j--;
            } else if (tab[i - 1][j] > tab[i][j - 1]) {
                i--;
            } else {
                j--;
            }
        }

        return result.toString();
    }






















    
    private static void BenchNaive(String S1, String S2) {
        long startTime = System.nanoTime();
        String result = PLSSC(S1, S2);
        long endTime = System.nanoTime();

        String time = String.valueOf((endTime - startTime) / 1.0E9);

        // System.out.println("PLSSC: " + result);
        System.out.println("Temps pour version naive: " + time + " secondes");

    }

    private static void BenchPd(String S1, String S2, int resultTaille, String result) {
        long startTime = System.nanoTime();
        String currentResult = PLSSC_PD(S1, S2);
        long endTime = System.nanoTime();

        String time = String.valueOf((endTime - startTime) / 1.0E6);

        System.out.println("Time: " + time + " millis");
        
        if (!currentResult.equals(result)) {
            System.out.println("Error: " + currentResult + " Attendu: " + result);
        } else {
            System.out.println("Sucess");
        }
    }

    public static void main(String args[]) {

        String S1;
        String S2;

        FileInputStream input;
        BufferedReader reader;

        for (int i = 0; i < args.length; i++) {
            try {
                // Ouverture du fichier passé en argument
                input = new FileInputStream(args[i]);
                reader = new BufferedReader(new InputStreamReader(input));

                S1 = reader.readLine();
                S2 = reader.readLine();

                int resultTaille = Integer.parseInt(reader.readLine());
                String result = reader.readLine();

                System.out.println("Taille de la premiere chaine: " + S1.length() + "\tTaille de la seconde chaine: "
                        + S2.length());

                //BenchNaive(S1, S2);
                BenchPd(S1, S2, resultTaille, result);

            } catch (FileNotFoundException e) {
                System.err.println("Erreur lors de l'ouverture du fichier " + args[i]);
            } catch (IOException e) {
                System.err.println("Erreur de lecture dans le fichier");
            }
        }
    }
}
