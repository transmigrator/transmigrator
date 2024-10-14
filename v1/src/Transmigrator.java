import java.awt.Desktop;
import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.net.URI;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

import org.bouncycastle.jsse.provider.BouncyCastleJsseProvider;

import javafx.application.Application;
import javafx.stage.Stage;

public class Transmigrator extends Application {
    private ProxyMesh proxyMesh;
    private ProxyMeshConfig proxyMeshConfig;

    @Override
    public void start(Stage primaryStage) {
        displayLoadingScreen(primaryStage);

        // Pause for a few seconds to display the loading screen
        PauseTransition pause = new PauseTransition(Duration.seconds(3));
        pause.setOnFinished(event -> {
            createProxyMeshConfig(primaryStage);
            createProxyMesh();
            createGUI(primaryStage);
        });
        pause.play();
    }

    private void displayLoadingScreen(Stage primaryStage) {
        // Create a BorderPane as the root node
        BorderPane root = new BorderPane();
        root.setStyle("-fx-background-color: black;");

        // Create an ImageView to display the image
        Image image = new Image(getClass().getResourceAsStream("/loading_screen.svg"));
        ImageView imageView = new ImageView(image);
        imageView.setFitHeight(primaryStage.getHeight());
        imageView.setPreserveRatio(true);

        // Add the ImageView to the center of the BorderPane
        root.setCenter(imageView);

        // Create a new Scene and set it as the content of the Stage
        Scene scene = new Scene(root, primaryStage.getWidth(), primaryStage.getHeight());
        scene.getRoot().setStyle("-fx-background-color: black; -fx-text-fill: white;");
        primaryStage.setScene(scene);
    }

    private void createProxyMeshConfig(Stage primaryStage) {
        // Create a FileManager instance
        FileManager fileManager = new FileManager(new File(System.getProperty("user.home")));

        // Use the FileManager to select the proxy list file
        File proxyListFile = new File(fileManager.selectFileForUpload());

        if (proxyListFile != null) {
            try {
                // Read proxy list from file
                List<String> proxyList = Files.readAllLines(Paths.get(proxyListFile.toURI()));

                // Validate the proxy list
                for (String proxy : proxyList) {
                    if (!proxy.matches("^[a-zA-Z0-9\\.\\:]+$")) {
                        System.err.println("Invalid proxy: " + proxy);
                        return;
                    }
                }

                // Create ProxyMeshConfig object
                proxyMeshConfig = new ProxyMeshConfig(proxyList.toArray(new String[0]), 3, proxyList.size());
            } catch (IOException e) {
                System.err.println("Error reading proxy list file: " + e.getMessage());
            }
        }
    }

    private void createProxyMesh() {
        // Create a BouncyCastleJsseProvider instance
        BouncyCastleJsseProvider provider = new BouncyCastleJsseProvider();

        // Create ProxyMesh object with the BouncyCastleJsseProvider instance
        proxyMesh = new ProxyMesh(provider);
    }

    private void createGUI(Stage primaryStage) {
        // Create a border pane layout
        BorderPane layout = new BorderPane();

        // Create a tab pane
        TabPane tabPane = new TabPane();

        // Create an address bar
        AddressBar addressBar = new AddressBar(new AddressBar.AddressBarCallback() {
            @Override
            public void onEnterPressed(String url, TabPane tabPane) {
                try {
                    // Use the proxy mesh to access the website
                    proxyMesh.processUrl(url);
                    // Create a new tab with the website content
                    Tab newTab = new Tab(url);
                    tabPane.getTabs().add(newTab);
                    // Set the new tab as the selected tab
                    tabPane.getSelectionModel().select(newTab);
                } catch (Exception e) {
                    // Handle any exceptions that might occur when processing the URL
                    System.err.println("Error processing URL: " + e.getMessage());
                }
            }

            @Override
            public void onCommandEntered(String command) {
                // Validate the command
                if (!command.matches("^[a-zA-Z0-9\\s]+$")) {
                    System.err.println("Invalid command: " + command);
                    return;
                }

                // Execute the command
                try {
                    Process process = Runtime.getRuntime().exec(command);
                    // Read the output of the command
                    BufferedReader reader = new BufferedReader(new java.io.InputStreamReader(process.getInputStream()));
                    String line;
                    while ((line = reader.readLine()) != null) {
                        System.out.println(line);
                    }
                } catch (IOException e) {
                    System.err.println("Error executing command: " + e.getMessage());
                }
            }

            @Override
            public void onError(String errorMessage) {
                System.err.println("Error: " + errorMessage);
            }
        });

        // Create a browser instance
        Browser browser = new Browser(tabPane, addressBar);

        // Create the GUI
        browser.createGUI(layout);

        // Set the layout as the content of the stage
        Scene scene = new Scene(layout);
        primaryStage.setScene(scene);
    }

    public static void main(String[] args) {
        System.setProperty("java.system.class.loader", Transmigrator.class.getName());
        launch(args);
    }
}
