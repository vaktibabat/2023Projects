import tensorflow.keras as tk
import cv2

model = tk.models.load_model("my_model")

cap = cv2.VideoCapture(0)

fac_exp_mapping = {
    0: 'Angry',
    1: 'Disgust',
    2: 'Fear',
    3: 'Happy',
    4: 'Sad',
    5: 'Surprise',
    6: 'Neutral'
}

if not cap.isOpened():
    raise IOError("Cannot open webcam")

while True:
    ret, frame = cap.read()

    frame = cv2.resize(frame, None, fx=1, fy=1, interpolation=cv2.INTER_AREA)

    cv2.imshow('Input', frame)

    c = cv2.waitKey(1)  

    if c == 27:
        break

    #Take the picture
    if c == 32:
        small_frame = cv2.resize(frame, (48, 48))
        small_frame = cv2.cvtColor(small_frame, cv2.COLOR_BGR2GRAY)
        small_frame = small_frame.reshape((1, 48, 48, 1))

        fac_exp = model.predict(small_frame)

        for i in range(0, 7):
            print("Probability of {}: {}".format(fac_exp_mapping[i], fac_exp[0][i]))

cap.release()
cv2.destroyAllWindows()
