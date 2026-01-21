import pandas as pd
from sklearn.preprocessing import LabelEncoder
from .constants import LABEL_COLUMNS, TEXT_COLUMN

def preprocess_dataset(df):
    df = df.copy()

    # Clean text
    df[TEXT_COLUMN] = (
        df[TEXT_COLUMN]
        .str.lower()
        .str.replace(r"\s+", " ", regex=True)
        .str.strip()
    )

    encoders = {}
    for col in LABEL_COLUMNS:
        le = LabelEncoder()
        df[f"{col}_id"] = le.fit_transform(df[col])
        encoders[col] = le

    return df, encoders
