import pandas as pd
from sklearn.cluster import KMeans
import numpy as np
import math
from tabulate import tabulate

dataset_with_genre = pd.read_csv('path/to/dataset')
dataset_without_genre = pd.read_csv('path/to/dataset')

genres = dataset_with_genre["genre"]

dup = []
num_of_genres = 0

for i in range(0, len(genres)):
    if genres[i] not in dup:
        num_of_genres += 1
        dup.append(genres[i])

kmeans = KMeans(n_clusters=num_of_genres)
y_pred = kmeans.fit_predict(dataset_without_genre)

cluster_labels = []

#ind[0] the index of the nearest, ind[1] the index of the second nearest, etc.
for cluster in range(0, num_of_genres):
    d = kmeans.transform(dataset_without_genre)[:, cluster]
    ind = np.argsort(d)[:5000]
    genre_dict = {}

    for genre in genres:
        genre_dict[genre] = 0

    for i in ind:
        genre_dict[genres[i]] += 1
    
    max_genre = None
    max_genre_cnt = 0

    for genre in genres:
        if genre_dict[genre] > max_genre_cnt:
            max_genre_cnt = genre_dict[genre]
            max_genre = genre


    second_max_genre = None
    second_max_genre_cnt = 0

    genre_dict[max_genre] = -1

    for genre in genres:
        if genre_dict[genre] > second_max_genre_cnt:
            second_max_genre_cnt = genre_dict[genre]
            second_max_genre = genre

    genre_dict[second_max_genre] = -1

    third_max_genre = None
    third_max_genre_cnt = 0

    for genre in genres:
        if genre_dict[genre] > third_max_genre_cnt:
            third_max_genre_cnt = genre_dict[genre]
            third_max_genre = genre
    
    cluster_labels.append((cluster, max_genre, second_max_genre, third_max_genre))

#Both have 13 coordinates: sqrt((x_1 - y_1)^2 + ... + (x_13 - y_13)^2)
def cluster_center_dist(v_1, v_2):
    sum = 0

    for i in range(0, 13):
        sum += (v_1[i] - v_2[i]) ** 2
    
    return math.sqrt(sum)

dist_matrix = []

for cluster_1 in range(0, num_of_genres):
    curr_row = []
    
    for cluster_2 in range(0, num_of_genres):
        curr_row.append(cluster_center_dist(kmeans.cluster_centers_[cluster_1], kmeans.cluster_centers_[cluster_2]))
    
    dist_matrix.append(curr_row)

data = []

for row_title in cluster_labels:
    row_data = []

    row_data.append("Cluster {}: {}, {}, and {}".format(row_title[0], row_title[1], row_title[2], row_title[3]))

    for j in range(0, num_of_genres):
        row_data.append(dist_matrix[row_title[0]][j])
    
    data.append(row_data)

col_titles = []

for col_title in cluster_labels:
    col_titles.append("Cluster {}: {}, {}, and {}".format(col_title[0], col_title[1], col_title[2], col_title[3]))

print(tabulate(data, headers=col_titles, tablefmt="grid"))
#print(dataset_with_genre.iloc[ind])