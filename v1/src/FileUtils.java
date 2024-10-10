import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;
import java.io.File;
import java.io.IOException;

public class FileUtils {
    public static String selectFile() {
        return FileChooser.selectFile();
    }

    public static String selectFileJ2ME() {
        try {
            FileConnection fc = (FileConnection) Connector.open("file:///root1/");
            if (fc.isDirectory()) {
                Enumeration fileEnum = fc.list();
                Vector fileVector = new Vector();
                while (fileEnum.hasMoreElements()) {
                    String file = (String) fileEnum.nextElement();
                    if (file.endsWith(".txt")) {
                        fileVector.addElement(file);
                    }
                }
                String selectedFile = (String) fileVector.elementAt(0);
                fc.close();
                return selectedFile;
            } else {
                fc.close();
                return null;
            }
        } catch (Exception e) {
            return null;
        }
    }

    public static String selectFileJVM() {
        // This method is no longer needed, as FileChooser handles JVM file selection
        return FileChooser.selectFile();
    }

    public static void saveFile(String fileName, String fileContent) {
        if (isJ2ME()) {
            saveFileJ2ME(fileName, fileContent);
        } else {
            saveFileJVM(fileName, fileContent);
        }
    }

    private static boolean isJ2ME() {
        return System.getProperty("microedition.configuration") != null;
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
            // Handle exception
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
            // Handle exception
        }
    }
}
