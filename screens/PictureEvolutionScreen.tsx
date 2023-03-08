import React, { useEffect, useState } from "react";
import { Button, FlatList, Image, Text, View } from "react-native";
import * as ImagePicker from "expo-image-picker";
import {
  addImage,
  ImageEntry,
  refreshImageEntries,
} from "../logic/PictureEvolutionLogic";
import PictureEvolutionStyles from "../styles/PictureEvolutionStyles";
import CommonStyles from "../styles/CommonStyles";
import * as PictureEvText from "../assets/texts/PictureEvolutionText";

export const PictureEvolutionScreen: React.FC = () => {
  const [imageList, setImageList] = useState<ImageEntry[]>([]);

  useEffect(() => {
    refreshImageEntries(setImageList);
  }, [pickImage]);

  return (
    <>
      <Button title={PictureEvText.pickItem} onPress={pickImage} />
      <FlatList
        data={imageList}
        renderItem={({ item }) => <ImageItem {...item} />}
        keyExtractor={(item: ImageEntry) => String(item.id ? item.id : 0)}
        style={CommonStyles.container}
      />
    </>
  );
};
const pickImage = async () => {
  let result = await ImagePicker.launchImageLibraryAsync({
    mediaTypes: ImagePicker.MediaTypeOptions.All,
    allowsEditing: true,
    aspect: [9, 16],
    quality: 1,
  });

  if (!result.canceled) {
    addImage(result.assets[0].uri);
  }
};

const ImageItem = ({ id, image, date }: ImageEntry) => (
  <View style={PictureEvolutionStyles.itemContainer}>
    <Image source={{ uri: image }} style={PictureEvolutionStyles.itemImage} />
    <Text style={PictureEvolutionStyles.itemDate}>{date.toDateString()}</Text>
  </View>
);
