# image-register-rs

This rust library provides capabilities for image registration via phase correlation.

<img src="./overview.svg" width="30%" alt="overview image">

## API

```
```

## testing

1. From the example image, creates cropped and offset images at test_images/translated.
2. Runs registration and merging and outputs the registered metrics and merged image to test_images/registered.
```
cargo test
```

# TODO
- Make api struct based, with defaults
- add quality control metrics: Peak to secondary peak ratio (PSPR) and Peak Z-score
- add hanning window option for edge artifacts
- add sub pixel registration option
- [OpenCV uses 5x5 weighted centroids for sub-pixel accuracy](https://docs.opencv.org/4.x/d7/df3/group__imgproc__motion.html#ga552420a2ace9ef3fb053cd630fdb4952).
