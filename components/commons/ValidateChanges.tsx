import { Alert } from "react-native";
import * as Text from "../../assets/texts/Common";

export function confirmationChanges(onPress: () => void) {
  Alert.alert(Text.confirm, Text.confirmChanges, [
    {
      text: Text.cancel,
      style: "cancel",
    },
    {
      text: "OK",
      onPress: () => {
        onPress();
      },
    },
  ]);
}
