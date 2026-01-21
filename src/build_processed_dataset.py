import pandas as pd
from preprocessing import preprocess_dataset
from constants import DATA_PATH_RAW, DATA_PATH_PROCESSED

def main():
    print("Loading raw dataset...")
    df = pd.read_csv(DATA_PATH_RAW)

    print("Preprocessing dataset...")
    df_processed, encoders = preprocess_dataset(df)

    print("Saving processed dataset...")
    df_processed.to_csv(DATA_PATH_PROCESSED, index=False)

    print("Processed dataset saved to:", DATA_PATH_PROCESSED)
    print("Label encoders created for:", list(encoders.keys()))

if __name__ == "__main__":
    main()
