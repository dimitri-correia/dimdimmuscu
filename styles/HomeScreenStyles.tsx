import { StyleSheet } from "react-native";

export default StyleSheet.create({
  row: {
    flexDirection: "row",
    justifyContent: "space-around",
  },
  item: {
    width: "40%",
    aspectRatio: 1,
    backgroundColor: "white",
    borderRadius: 40,
    justifyContent: "center",
    alignItems: "center",
  },
  itemImage: {
    width: "60%",
    height: "60%",
    resizeMode: "contain",
  },
  itemText: {
    fontSize: 15,
    fontWeight: "bold",
    textAlign: "center",
  },
});
