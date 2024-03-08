import { ReactNode, useEffect, useRef } from 'react';

import styles from './list.module.scss';
import NotFoundSVG from './not-found.svg?react';

type Props<T> = {
  items: T[];
  itemsPerRow: number;
  emptyText: string;
  isMoreItems: boolean;
  renderItem: (item: T, index: number) => ReactNode;
  fetchItems: () => void;

  skeleton: {
    rowsCount: number;
    isVisible: boolean;
    renderItem: (index: number) => ReactNode;
  };
};

function Observer({ fetch }: { fetch: () => void }) {
  const observerRef = useRef<HTMLSpanElement>(null);

  useEffect(() => {
    if (!observerRef.current) return;

    const observer = new IntersectionObserver(([{ isIntersecting }]) => {
      if (!isIntersecting) return;

      fetch();
      console.log('fetch');
    });

    observer.observe(observerRef.current);

    return () => {
      observer.disconnect();
    };
  }, [fetch]);

  return <span ref={observerRef} />;
}

function List<T>({ items, itemsPerRow, emptyText, isMoreItems, renderItem, fetchItems, skeleton }: Props<T>) {
  const itemsCount = items.length;
  const isListVisible = Boolean(itemsCount) || skeleton.isVisible;

  const renderItems = () => items.map((item, index) => renderItem(item, index));

  const renderSkeletonItems = () =>
    new Array(itemsPerRow * skeleton.rowsCount).fill(null).map((_, index) => skeleton.renderItem(index));

  return isListVisible ? (
    <div>
      <ul className={styles.list} style={{ gridTemplateColumns: `repeat(${itemsPerRow}, 1fr)` }}>
        {renderItems()}
        {skeleton.isVisible && renderSkeletonItems()}
      </ul>

      {isMoreItems && !skeleton.isVisible && <Observer fetch={fetchItems} />}
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

export { List };
