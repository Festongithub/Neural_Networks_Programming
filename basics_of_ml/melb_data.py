import pandas as pd
from sklearn.tree import DecisionTreeRegressor

melb_file_path = '/Users/macbookair/Neural_Networks_Programming/basics_of_ml/melb_data.csv'

melbourne_data = pd.read_csv(melb_file_path)
for i in range(len(melbourne_data.columns)):
    print(i, melbourne_data.columns[i])


# selecting the prediction target

y = melbourne_data.Price

# Choosing Features

melbourne_features = ['Rooms', 'Bathroom', 'Landsize', 'Lattitude', 'Longtitude']

X = melbourne_data[melbourne_features]

#print(X.describe)
#print(X.head())

melbourne_model = DecisionTreeRegressor(random_state=1)

melbourne_model.fit(X, y)
