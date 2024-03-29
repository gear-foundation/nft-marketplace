import { Button } from '@gear-js/vara-ui';

import { cx } from '@/utils';

import TrashSVG from '../../assets/trash.svg?react';

import styles from './delete-button.module.scss';

type Props = {
  className?: string;
  onClick: () => void;
};

function DeleteButton({ className, onClick }: Props) {
  return <Button icon={TrashSVG} color="transparent" className={cx(styles.button, className)} onClick={onClick} />;
}

export { DeleteButton };
