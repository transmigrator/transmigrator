import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.OutputStream;
import java.util.Enumeration;
import java.util.Vector;

public class FileUtils {
    public static boolean isJ2ME() {
        try {
            Class.forName("javax.microedition.io.Connector");
            return true;
        } catch (ClassNotFoundException e) {
            return false;
        }
    }

    public static String selectFileJ2ME() {
        try {
            FileConnection fc = (FileConnection) Connector.open("file:///root1/");
            if (fc.isDirectory()) {
                Enumeration fileEnum = fc.list();
                Vector fileVector = new Vector();
                while (fileEnum.hasMoreElements()) {
                    String file = (String) fileEnum.nextElement();
                    fileVector.addElement(file);
                }
                // Create a file chooser to select a file
                FileChooser fileChooser = new FileChooser();
                fileChooser.setFiles(fileVector);
                int selectedIndex = fileChooser.showFileChooser();
                if (selectedIndex != -1) {
                    String selectedFile = (String) fileVector.elementAt(selectedIndex);
                    fc.close();
                    return selectedFile;
                } else {
                    fc.close();
                    return null;
                }
            } else {
                fc.close();
                return null;
            }
        } catch (Exception e) {
            return null;
        }
    }

    public static String selectFileJVM() {
        JFileChooser fileChooser = new JFileChooser();
        int returnValue = fileChooser.showOpenDialog(null);
        if (returnValue == JFileChooser.APPROVE_OPTION) {
            File selectedFile = fileChooser.getSelectedFile();
            return selectedFile.getAbsolutePath();
        } else {
            return null;
        }
    }

    public static void saveFile(String fileName, String fileContent) {
        if (isJ2ME()) {
            saveFileJ2ME(fileName, fileContent);
        } else {
            saveFileJVM(fileName, fileContent);
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
        } catch (IOException e) {
            // Handle exception, e.g., log the error or display an error message
            System.out.println("Error saving file: " + e.getMessage());
        } catch (SecurityException e) {
            // Handle exception, e.g., log the error or display an error message
            System.out.println("Security error saving file: " + e.getMessage());
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
            // Handle exception, e.g., log the error or display an error message
            System.out.println("Error saving file: " + e.getMessage());
        } catch (SecurityException e) {
            // Handle exception, e.g., log the error or display an error message
            System.out.println("Security error saving file: " + e.getMessage());
        }
    }
}
