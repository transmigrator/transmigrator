import javafx.scene.control.TextField;
import javafx.scene.input.KeyCode;

/**
 * Represents an address bar with a text field and a callback interface.
 */
public class AddressBar {
    private TextField addressBar;
    private AddressBarCallback callback;

    /**
     * Constructs a new AddressBar object with the given callback.
     *
     * @param callback the callback interface to handle the enter key press event
     */
    public AddressBar(AddressBarCallback callback) {
        if (callback == null) {
            throw new NullPointerException("Callback cannot be null");
        }
        this.callback = callback;
        addressBar = new TextField();
        addressBar.setPromptText("Enter URL or command");
        addressBar.setOnKeyPressed(event -> {
            if (event.getCode() == KeyCode.ENTER) {
                String input = addressBar.getText();
                if (isUrl(input)) {
                    try {
                        callback.onEnterPressed(input);
                    } catch (Exception e) {
                        // Handle any exceptions that might occur when processing the URL
                        callback.onError(e.getMessage());
                    }
                } else {
                    // Handle terminal command
                    callback.onCommandEntered(input);
                }
            }
        });
    }

    /**
     * Returns the text field of the address bar.
     *
     * @return the text field of the address bar
     */
    public TextField getAddressBar() {
        return addressBar;
    }

    /**
     * Checks if the input is a URL.
     *
     * @param input the input to check
     * @return true if the input is a URL, false otherwise
     */
    private boolean isUrl(String input) {
        try {
            java.net.URI uri = new java.net.URI(input);
            return uri.getScheme() != null && uri.getHost() != null;
        } catch (Exception e) {
            return false;
        }
    }

    /**
     * Callback interface to handle the enter key press event.
     */
    public interface AddressBarCallback {
        /**
         * Called when the enter key is pressed in the address bar with a URL.
         *
         * @param url the URL entered in the address bar
         */
        void onEnterPressed(String url);

        /**
         * Called when the enter key is pressed in the address bar with a terminal command.
         *
         * @param command the terminal command entered in the address bar
         */
        void onCommandEntered(String command);

        /**
         * Called when an error occurs while processing the URL.
         *
         * @param errorMessage the error message
         */
        void onError(String errorMessage);
    }
}