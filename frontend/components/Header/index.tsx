import type { NextPage } from "next";
import React, { useEffect, useState } from "react";
import { Input } from "antd";
import Link from "next/link";
import Image from "next/image";

const { Search } = Input;

const Header: NextPage = () => {
  const onSearch = (value: string) => console.log(value);

  return (
    <div className="py-40px header-container wrapper flex justify-between">
      <div>
        <Link href="/blocks">
          <Image
            src="/images/logo_with_text.png"
            alt="Picture of the author"
            width={98}
            height={40}
            className="cursor-pointer"
          />
        </Link>
      </div>
      <Search
        placeholder="Search by Block, Account, Public key or TX"
        allowClear
        onSearch={onSearch}
        className="!w-400px"
      />
    </div>
  );
};

export default Header;
