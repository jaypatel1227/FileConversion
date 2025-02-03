import React, { useState } from "react";
import { ConvertRequestButton } from "./ConvertRequestButton"

export interface ICardProps {
  name: string;
  description: string;
  post_request_url: string;
  from_extension: string;
  to_extension?: string;
}

const Card: React.FC<ICardProps> = (props) => {
  const [waiting, setWaiting] = useState(false);
  let { name, description, from_extension, post_request_url } = props;

  return (
    <div className="_container">
      <div className="_cardTitle"> {name} </div>
      <div className="_flexBreak"> </div>
      <div className="_cardDescription"> {description} </div>
      <div className="_flexBreak"> </div>
      <div className="_formArea" hidden={waiting}>
        <ConvertRequestButton name={name} from_extension={from_extension} post_request_url={post_request_url} setWaiting={setWaiting} />
      </div>
      <div hidden={!waiting}>
        Waiting for response...
      </div>
    </div>
  );
};

interface ICardRowProps {
  cards: ICardProps[];
  initialIndex: number;
}

const CardRow: React.FC<ICardRowProps> = (props) => {

  return (
    <>
      {props.cards.map((card, index) => (
        <td key={index + props.initialIndex} className="_cardItem">
          <Card name={card.name} description={card.description} from_extension={card.from_extension} post_request_url={card.post_request_url} />
        </td>
      ))}
    </>
  );
};

function filterCards(cards: ICardProps[], searchFilter: string): ICardProps[] {
  searchFilter = searchFilter.toLowerCase();
  return cards.filter((card) => card.name.toLowerCase().includes(searchFilter) || card.description.toLowerCase().includes(searchFilter));
}

export interface ICardTableProps {
  cards?: ICardProps[],
  searchFilter: string,
}

export const CardTable: React.FC<ICardTableProps> = (props: ICardTableProps) => {

  let NUM_COLS = 3;

  if (!props.cards) {
    return null;
  }

  const filteredCards = !props.searchFilter ? props.cards : filterCards(props.cards, props.searchFilter);

  const groupedCards = filteredCards.reduce<ICardProps[][]>((arr, card, index) => {
    if (index % NUM_COLS === 0) arr.push([]);
    arr[arr.length - 1].push(card);
    return arr;
  }, []);

  return (
    <table className="_serviceGrid">
      <tbody>
        {groupedCards?.map((group, index) => (
          <tr className="_tableRow">
            <CardRow cards={group} initialIndex={index * NUM_COLS} />
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default CardTable;