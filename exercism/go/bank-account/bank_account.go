package account

import "sync"

type Account struct {
	balance int
	closed  bool
	sync.Mutex
}

func Open(amount int) *Account {
	if amount < 0 {
		return nil
	}
	return &Account{balance: amount, closed: false}

}

func (ac *Account) Deposit(amount int) (newBalance int, ok bool) {
	if !ac.closed {
		ac.Lock()
		defer ac.Unlock()

		if ac.balance+amount >= 0 {
			ac.balance += amount
			newBalance = ac.balance
			ok = true
		}
	}

	return
}

func (ac *Account) Balance() (int, bool) {
	return ac.balance, !ac.closed
}

func (ac *Account) Close() (int, bool) {
	ac.Lock()
	defer ac.Unlock()
	if ac.closed {
		return 0, false
	}
	payout := ac.balance
	ac.balance = 0
	ac.closed = true
	return payout, ac.closed
}
