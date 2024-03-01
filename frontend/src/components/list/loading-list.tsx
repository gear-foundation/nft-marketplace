import { ReactNode, useEffect, useRef } from 'react';

import styles from './list.module.scss';
import NotFoundSVG from './not-found.svg?react';

type Props<T> = {
  items: T[];
  itemsPerRow: number;
  emptyText: string;
  renderItem: (item: T, index: number) => ReactNode;

  skeleton: {
    rowsCount: number;
    isVisible: boolean;
    renderItem: (index: number) => ReactNode;
  };
};

function LoadingList<T>({ items, itemsPerRow, emptyText, renderItem, skeleton }: Props<T>) {
  const itemsCount = items.length;
  const isListVisible = Boolean(itemsCount) || skeleton.isVisible;

  const renderItems = () => items.map((item, index) => renderItem(item, index));

  const renderSkeletonItems = () => {
    const lastRowRemainder = itemsCount % itemsPerRow;
    const maxSkeletonsCount = itemsPerRow * skeleton.rowsCount;
    const skeletonsCount = maxSkeletonsCount - lastRowRemainder;

    return new Array(skeletonsCount).fill(null).map((_, index) => skeleton.renderItem(index));
  };

  const observerRef = useRef<HTMLSpanElement>(null);

  useEffect(() => {
    if (!observerRef.current) return;

    const observer = new IntersectionObserver(([{ isIntersecting }]) => {
      if (!isIntersecting) return;

      console.log('fetch');
    });

    observer.observe(observerRef.current);

    return () => {
      observer.disconnect();
    };
  }, []);

  return isListVisible ? (
    <div>
      <ul className={styles.list} style={{ gridTemplateColumns: `repeat(${itemsPerRow}, 1fr)` }}>
        {renderItems()}
        {skeleton.isVisible && renderSkeletonItems()}
      </ul>

      <span ref={observerRef} />
    </div>
  ) : (
    <div className={styles.notFound}>
      <NotFoundSVG />

      <h3 className={styles.heading}>Oops, Nothing Found!</h3>
      <p className={styles.text}>
        Looks like we&apos;re on a wild goose chase! {emptyText} to have them displayed here.
      </p>
    </div>
  );
}

export { LoadingList };
