from sklearn.linear_model import LogisticRegression

def build_classifier():
    return LogisticRegression(
        max_iter=1000,
        class_weight="balanced"
    )
