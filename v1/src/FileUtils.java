import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.OutputStream;

import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;

public class FileUtils {
    public static void saveFile(String fileName, String fileContent) {
        if (isJ2ME()) {
            saveFileJ2ME(fileName, fileContent);
        } else {
            saveFileJVM(fileName, fileContent);
        }
    }

    private static boolean isJ2ME() {
        try {
            Class.forName("javax.microedition.io.Connector");
            return true;
        } catch (ClassNotFoundException e) {
            // Log the error or handle it in some way
            System.out.println("Error loading javax.microedition.io.Connector: " + e.getMessage());
            return false;
        }
    }

    private static void saveFileJ2ME(String fileName, String fileContent) {
        try {
            FileConnection fc = (FileConnection) Connector.open("file:///" + fileName);
            if (!fc.exists()) {
                fc.create();
            }
            OutputStream os = fc.openOutputStream();
            os.write(fileContent.getBytes());
            os.close();
            fc.close();
        } catch (Exception e) {
            // Log the error or handle it in some way
            System.out.println("Error saving file: " + e.getMessage());
        }
    }

    private static void saveFileJVM(String fileName, String fileContent) {
        try {
            File file = new File(fileName);
            if (!file.exists()) {
                file.createNewFile();
            }
            FileWriter fw = new FileWriter(file);
            fw.write(fileContent);
            fw.close();
        } catch (IOException e) {
            // Log the error or handle it in some way
            System.out.println("Error saving file: " + e.getMessage());
        } catch (SecurityException e) {
            // Log the error or handle it in some way
            System.out.println("Security error saving file: " + e.getMessage());
        }
    }

    public static String getFileContent(String fileName) {
        if (isJ2ME()) {
            return getFileContentJ2ME(fileName);
        } else {
            return getFileContentJVM(fileName);
        }
    }

    private static String getFileContentJ2ME(String fileName) {
        try {
            FileConnection fc = (FileConnection) Connector.open("file:///" + fileName);
            if (fc.exists()) {
                int fileSize = (int) fc.fileSize();
                byte[] fileContentBytes = new byte[fileSize];
                InputStream is = fc.openInputStream();
                is.read(fileContentBytes);
                is.close();
                fc.close();
                return new String(fileContentBytes);
            } else {
                fc.close();
                return null;
            }
        } catch (Exception e) {
            // Log the error or handle it in some way
            System.out.println("Error getting file content: " + e.getMessage());
            return null;
        }
    }

    private static String getFileContentJVM(String fileName) {
        try {
            File file = new File(fileName);
            if (file.exists()) {
                FileReader fr = new FileReader(file);
                char[] fileContentChars = new char[(int) file.length()];
                fr.read(fileContentChars);
                fr.close();
                return new String(fileContentChars);
            } else {
                return null;
            }
        } catch (IOException e) {
            // Log the error or handle it in some way
            System.out.println("Error getting file content: " + e.getMessage());
            return null;
        } catch (SecurityException e) {
            // Log the error or handle it in some way
            System.out.println("Security error getting file content: " + e.getMessage());
            return null;
        }
    }
}
