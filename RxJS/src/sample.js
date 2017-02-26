const source = Rx.Observable.create((observer) => {
  let num = 0;
  const id = setInterval(() => {
    observer.next(num++);
  }, 1000);
  
  setTimeout(() => {
    observer.complete();
  }, 10000);
  
  return () => {
    console.log('disposed');
    clearInterval(id);
  };
});

const subscription = source.subscribe(
  (x) => {
    console.log('onNext: ' + x);
  },
  (e) => {
    console.log('onError: ' + e.message);
  },
  () => {
    console.log('onCompleted');
  }
);

setTimeout(() => {
  subscription.unsubscribe();
}, 5000);
