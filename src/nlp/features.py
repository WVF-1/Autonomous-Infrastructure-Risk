from sklearn.feature_extraction.text import TfidfVectorizer

def build_tfidf(max_features=5000, ngram_range=(1,2)):
    return TfidfVectorizer(
        max_features=max_features,
        ngram_range=ngram_range,
        stop_words="english"
    )
